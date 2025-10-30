//! Resource modules

pub mod location;
pub use location::Location;
pub mod well_known;
pub use well_known::Well_known;
pub mod usable_subnetwork;
pub use usable_subnetwork::Usable_subnetwork;
pub mod cluster;
pub use cluster::Cluster;
pub mod node_pool;
pub use node_pool::Node_pool;
pub mod operation;
pub use operation::Operation;
pub mod zone;
pub use zone::Zone;

