//! Resource modules

pub mod backup;
pub use backup::Backup;
pub mod location;
pub use location::Location;
pub mod sql_integration;
pub use sql_integration::Sql_integration;
pub mod domain;
pub use domain::Domain;
pub mod peering;
pub use peering::Peering;
pub mod operation;
pub use operation::Operation;

