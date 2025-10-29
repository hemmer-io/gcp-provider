//! Resource modules

pub mod operation;
pub use operation::Operation;
pub mod backup;
pub use backup::Backup;
pub mod location;
pub use location::Location;
pub mod instance;
pub use instance::Instance;
pub mod snapshot;
pub use snapshot::Snapshot;
pub mod share;
pub use share::Share;

