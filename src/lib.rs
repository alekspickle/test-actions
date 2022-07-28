pub mod defs;
pub mod error;
pub mod cli;
pub mod server;

pub use error::*;
pub use defs::*;
pub use cli::*;
pub use server::*;

use octocrab::Octocrab;

pub type Result<T = (), E = Box<dyn std::error::Error + Send + Sync>> = std::result::Result<T, E>;
