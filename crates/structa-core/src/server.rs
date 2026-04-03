use std::any::Any;
use crate::di::{Container, InjectionToken, Module, Provider};
use crate::errors::Result;
use crate::plugins::Plugin;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub struct StructaApp {
    container: Container,
    modules: Vec<Box<dyn Module>>,
    plugins: Vec<Box<dyn Plugin>>,
    config: AppConfig,
}

#[derive(Debug, Clone, Default)]
pub struct AppConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub env: String,
}

impl Default for StructaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl StructaApp {
    pub fn new() -> Self {
        Self {
            container: Container::new(),
            modules: Vec::new(),
            plugins: Vec::new(),
            config: AppConfig::default(),
        }
    }

    pub fn configure<F>(mut self, f: F) -> Self 
    where 
        F: FnOnce(&mut AppConfig),
    {
        f(&mut self.config);
        self
    }

    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.config.name = name.into();
        self
    }

    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.config.host = host.into();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn env(mut self, env: impl Into<String>) -> Self {
        self.config.env = env.into();
        self
    }

    pub fn module<M: Module + 'static>(mut self, module: M) -> Self {
        self.modules.push(Box::new(module));
        self
    }

    pub fn plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn provider<T: 'static>(&self, token: InjectionToken, provider: Provider) -> Result<&Self> {
        self.container.register(&token, provider)?;
        Ok(self)
    }

    pub fn register<T: Any + Send + Sync + Clone + 'static>(&self, token: InjectionToken, value: T) -> Result<&Self> {
        self.container.register(&token, value)?;
        Ok(self)
    }

    pub async fn init(self) -> Result<StructaInstance> {
        info!("Initializing Structa application: {}", self.config.name);

        let instance = StructaInstance::new(self.container.clone(), self.config.clone());

        for plugin in &self.plugins {
            info!("Running plugin: {}", plugin.name());
            plugin.setup(&instance).await?;
        }

        for module in &self.modules {
            info!("Bootstrapping module");
            let _ = module;
        }

        info!("Application initialized successfully");
        Ok(instance)
    }

    pub async fn listen(self) -> Result<()> {
        let instance = self.init().await?;
        instance.start().await
    }
}

#[derive(Clone)]
pub struct StructaInstance {
    container: Container,
    config: AppConfig,
    state: Arc<RwLock<InstanceState>>,
}

struct InstanceState {
    running: bool,
    started_at: Option<std::time::Instant>,
}

impl StructaInstance {
    fn new(container: Container, config: AppConfig) -> Self {
        Self {
            container,
            config,
            state: Arc::new(RwLock::new(InstanceState {
                running: false,
                started_at: None,
            })),
        }
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn is_running(&self) -> bool {
        self.state.read().running
    }

    pub async fn start(&self) -> Result<()> {
        {
            let mut state = self.state.write();
            if state.running {
                return Err(crate::errors::StructaError::Internal("Server already running".to_string()));
            }
            state.running = true;
            state.started_at = Some(std::time::Instant::now());
        }

        info!("Server listening on {}:{}", self.config.host, self.config.port);
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let mut state = self.state.write();
        if !state.running {
            return Err(crate::errors::StructaError::Internal("Server not running".to_string()));
        }
        state.running = false;
        info!("Server stopped");
        Ok(())
    }
}

pub fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

pub fn init_logging_with_level(level: Level) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}
