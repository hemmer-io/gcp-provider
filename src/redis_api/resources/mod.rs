//! Resource modules

pub mod cluster;
pub use cluster::Cluster;
pub mod instance;
pub use instance::Instance;
pub mod backup_collection;
pub use backup_collection::Backup_collection;
pub mod operation;
pub use operation::Operation;
pub mod backup;
pub use backup::Backup;
pub mod location;
pub use location::Location;

