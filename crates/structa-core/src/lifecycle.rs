use async_trait::async_trait;
use crate::server::StructaInstance;
use crate::errors::Result;

#[async_trait]
pub trait LifecycleHook: Send + Sync {
    async fn on_start(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }

    async fn on_stop(&self, _instance: &StructaInstance) -> Result<()> {
        Ok(())
    }
}

pub trait OnStart: Send + Sync {
    fn on_start(&self) -> impl std::future::Future<Output = crate::errors::Result<()>> + Send;
}

pub trait OnStop: Send + Sync {
    fn on_stop(&self) -> impl std::future::Future<Output = crate::errors::Result<()>> + Send;
}
