//! Resource modules

pub mod workload;
pub use workload::Workload;
pub mod location;
pub use location::Location;
pub mod discovered_service;
pub use discovered_service::Discovered_service;
pub mod service_project_attachment;
pub use service_project_attachment::Service_project_attachment;
pub mod operation;
pub use operation::Operation;
pub mod application;
pub use application::Application;
pub mod discovered_workload;
pub use discovered_workload::Discovered_workload;
pub mod service;
pub use service::Service;

