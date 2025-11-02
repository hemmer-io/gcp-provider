//! Resource modules

pub mod deployment;
pub use deployment::Deployment;
pub mod revision;
pub use revision::Revision;
pub mod resource_change;
pub use resource_change::Resource_change;
pub mod resource_drift;
pub use resource_drift::Resource_drift;
pub mod operation;
pub use operation::Operation;
pub mod location;
pub use location::Location;
pub mod resource;
pub use resource::Resource;
pub mod terraform_version;
pub use terraform_version::Terraform_version;
pub mod preview;
pub use preview::Preview;

