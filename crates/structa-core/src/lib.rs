pub mod errors;
pub mod logger;
pub mod types;

pub use errors::{Result, StructaError};
pub use logger::Logger;
pub use types::{CompilationError, ModuleType, SourceLocation};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
