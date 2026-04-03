use async_trait::async_trait;
use crate::errors::Result;
use crate::server::StructaInstance;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    
    async fn setup(&self, instance: &StructaInstance) -> Result<()>;
    
    async fn before_start(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }

    async fn after_start(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }

    async fn before_stop(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }

    async fn after_stop(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct PluginContext {
    metadata: HashMap<String, String>,
    services: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl PluginContext {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
            services: HashMap::new(),
        }
    }

    pub fn set_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(|s| s.as_str())
    }

    pub fn set_service<T: Any + Send + Sync + 'static>(&mut self, key: impl Into<String>, service: T) {
        self.services.insert(key.into(), Arc::new(service));
    }

    pub fn get_service<T: Any + Send + Sync + 'static>(&self, key: &str) -> Option<&T> {
        self.services.get(key).and_then(|s| s.downcast_ref::<T>())
    }
}

pub struct PluginManager {
    plugins: RwLock<Vec<Box<dyn Plugin>>>,
    contexts: RwLock<HashMap<String, PluginContext>>,
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: RwLock::new(Vec::new()),
            contexts: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<P: Plugin + 'static>(&self, plugin: P) {
        let name = plugin.name().to_string();
        self.contexts.write().insert(name, PluginContext::new());
        self.plugins.write().push(Box::new(plugin));
    }

    pub async fn init_all(&self, instance: &StructaInstance) -> Result<()> {
        for plugin in self.plugins.read().iter() {
            plugin.setup(instance).await?;
        }
        Ok(())
    }

    pub async fn run_lifecycle(&self, instance: &StructaInstance, stage: LifecycleStage) -> Result<()> {
        for plugin in self.plugins.read().iter() {
            match stage {
                LifecycleStage::BeforeStart => plugin.before_start(instance).await?,
                LifecycleStage::AfterStart => plugin.after_start(instance).await?,
                LifecycleStage::BeforeStop => plugin.before_stop(instance).await?,
                LifecycleStage::AfterStop => plugin.after_stop(instance).await?,
            }
        }
        Ok(())
    }

    pub fn get_context(&self, plugin_name: &str) -> Option<PluginContext> {
        self.contexts.read().get(plugin_name).cloned()
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.read().iter().map(|p| p.name().to_string()).collect()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LifecycleStage {
    BeforeStart,
    AfterStart,
    BeforeStop,
    AfterStop,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin;

    #[async_trait]
    impl Plugin for TestPlugin {
        fn name(&self) -> &str {
            "test_plugin"
        }

        async fn setup(&self, _instance: &StructaInstance) -> Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_plugin_manager() {
        let manager = PluginManager::new();
        manager.register(TestPlugin);
        
        let plugins = manager.list_plugins();
        assert_eq!(plugins, vec!["test_plugin"]);
    }
}
