use crate::errors::{Result, StructaError};
use parking_lot::RwLock;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InjectionToken {
    id: &'static str,
}

impl InjectionToken {
    pub const fn new<T: 'static>(id: &'static str) -> Self {
        Self { id }
    }

    pub fn with_id(id: &'static str) -> Self {
        Self { id }
    }

    pub fn identifier(&self) -> &'static str {
        self.id
    }
}

impl std::fmt::Display for InjectionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InjectionToken({})", self.id)
    }
}

pub trait Injectable: Send + Sync + 'static {}

pub struct Container {
    providers: RwLock<HashMap<String, Provider>>,
    instances: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Container {
    fn clone(&self) -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            instances: RwLock::new(HashMap::new()),
        }
    }
}

pub enum Provider {
    Factory(Arc<dyn Fn(&Container) -> Result<Arc<dyn Any + Send + Sync>> + Send + Sync>),
}

impl Clone for Provider {
    fn clone(&self) -> Self {
        match self {
            Provider::Factory(f) => Provider::Factory(f.clone()),
        }
    }
}

impl Container {
    pub fn new() -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            instances: RwLock::new(HashMap::new()),
        }
    }

    pub fn register<T: 'static + Send + Sync + Clone + Any>(
        &self,
        token: &InjectionToken,
        value: T,
    ) -> Result<()> {
        let key = token.identifier();
        self.instances
            .write()
            .insert(key.to_string(), Arc::new(value));
        Ok(())
    }

    pub fn register_factory<T: Send + Sync + 'static>(
        &self,
        token: &InjectionToken,
        factory: impl Fn(&Container) -> Result<T> + Send + Sync + 'static,
    ) -> Result<()>
    where
        T: 'static,
    {
        let key = token.identifier();
        let wrapped = Arc::new(move |c: &Container| {
            let result = factory(c)?;
            Ok(Arc::new(result) as Arc<dyn Any + Send + Sync>)
        });

        self.providers
            .write()
            .insert(key.to_string(), Provider::Factory(wrapped));
        Ok(())
    }

    pub fn resolve<T: 'static + Send + Sync + Clone + Any>(
        &self,
        token: &InjectionToken,
    ) -> Result<T> {
        let key = token.identifier();

        if let Some(value) = self.instances.read().get(key) {
            return value
                .clone()
                .downcast::<T>()
                .map(|b| (*b).clone())
                .map_err(|_| StructaError::Internal("Type mismatch".to_string()));
        }

        if let Some(provider) = self.providers.read().get(key).cloned() {
            match provider {
                Provider::Factory(factory) => {
                    let value = factory(self)?;
                    self.instances
                        .write()
                        .insert(key.to_string(), value.clone());
                    value
                        .downcast::<T>()
                        .map(|b| (*b).clone())
                        .map_err(|_| StructaError::Internal("Type mismatch".to_string()))
                }
            }
        } else {
            Err(StructaError::ProviderNotFound(key.to_string()))
        }
    }

    pub fn has(&self, token: &InjectionToken) -> bool {
        let key = token.identifier();
        self.providers.read().contains_key(key) || self.instances.read().contains_key(key)
    }
}

pub trait Module: Send + Sync {
    fn exports(&self) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_register_and_resolve() {
        let container = Container::new();
        let token = InjectionToken::new::<i32>("test_token");

        container.register(&token, 42).unwrap();
        let value: i32 = container.resolve(&token).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn test_injection_token() {
        let token1 = InjectionToken::new::<String>("token1");
        let token2 = InjectionToken::new::<i32>("token2");

        assert_eq!(token1.identifier(), "token1");
        assert_eq!(token2.identifier(), "token2");
    }
}
