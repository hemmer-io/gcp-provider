//! Resource modules

pub mod share;
pub use share::Share;
pub mod operation;
pub use operation::Operation;
pub mod location;
pub use location::Location;
pub mod instance;
pub use instance::Instance;
pub mod backup;
pub use backup::Backup;
pub mod snapshot;
pub use snapshot::Snapshot;

