//! Resource modules

pub mod backup_collection;
pub use backup_collection::Backup_collection;
pub mod location;
pub use location::Location;
pub mod instance;
pub use instance::Instance;
pub mod operation;
pub use operation::Operation;
pub mod cluster;
pub use cluster::Cluster;
pub mod backup;
pub use backup::Backup;

