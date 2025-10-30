//! Resource modules

pub mod version;
pub use version::Version;
pub mod instance;
pub use instance::Instance;
pub mod location;
pub use location::Location;
pub mod operation;
pub use operation::Operation;
pub mod namespace;
pub use namespace::Namespace;
pub mod dns_peering;
pub use dns_peering::Dns_peering;

