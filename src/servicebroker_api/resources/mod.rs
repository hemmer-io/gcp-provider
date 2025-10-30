//! Resource modules

pub mod service_binding;
pub use service_binding::Service_binding;
pub mod catalog;
pub use catalog::Catalog;
pub mod instance;
pub use instance::Instance;
pub mod service_instance;
pub use service_instance::Service_instance;
pub mod servicebroker;
pub use servicebroker::Servicebroker;

