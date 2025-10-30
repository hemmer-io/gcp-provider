//! Resource modules

pub mod operation;
pub use operation::Operation;
pub mod domain;
pub use domain::Domain;
pub mod location;
pub use location::Location;
pub mod backup;
pub use backup::Backup;
pub mod peering;
pub use peering::Peering;
pub mod sql_integration;
pub use sql_integration::Sql_integration;

