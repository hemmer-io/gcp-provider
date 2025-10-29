//! Resource modules

pub mod namespace;
pub use namespace::Namespace;
pub mod service;
pub use service::Service;
pub mod workload;
pub use workload::Workload;
pub mod location;
pub use location::Location;
pub mod endpoint;
pub use endpoint::Endpoint;

