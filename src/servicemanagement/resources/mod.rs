//! Resource modules

pub mod service;
pub use service::Service;
pub mod config;
pub use config::Config;
pub mod operation;
pub use operation::Operation;
pub mod rollout;
pub use rollout::Rollout;
pub mod consumer;
pub use consumer::Consumer;

