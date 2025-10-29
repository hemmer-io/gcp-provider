//! Resource modules

pub mod location;
pub use location::Location;
pub mod cluster;
pub use cluster::Cluster;
pub mod node_pool;
pub use node_pool::Node_pool;
pub mod well-known;
pub use well-known::Well-known;
pub mod zone;
pub use zone::Zone;
pub mod usable_subnetwork;
pub use usable_subnetwork::Usable_subnetwork;
pub mod operation;
pub use operation::Operation;

