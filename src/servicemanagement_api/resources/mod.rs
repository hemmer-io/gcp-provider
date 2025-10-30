//! Resource modules

pub mod consumer;
pub use consumer::Consumer;
pub mod config;
pub use config::Config;
pub mod service;
pub use service::Service;
pub mod rollout;
pub use rollout::Rollout;
pub mod operation;
pub use operation::Operation;

