pub mod di;
pub mod server;
pub mod plugins;
pub mod metadata;
pub mod errors;
pub mod lifecycle;

pub use di::{Container, Provider, InjectionToken, Injectable};
pub use server::{StructaApp, StructaInstance};
pub use plugins::{Plugin, PluginContext};
pub use metadata::{Metadata, ModuleMetadata};
pub use lifecycle::{LifecycleHook, OnStart, OnStop};
pub use errors::{StructaError, Result};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
