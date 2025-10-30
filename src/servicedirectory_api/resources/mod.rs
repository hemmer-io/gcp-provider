//! Resource modules

pub mod location;
pub use location::Location;
pub mod service;
pub use service::Service;
pub mod namespace;
pub use namespace::Namespace;
pub mod workload;
pub use workload::Workload;
pub mod endpoint;
pub use endpoint::Endpoint;

