//! Resource modules

pub mod vmware_cluster;
pub use vmware_cluster::Vmware_cluster;
pub mod vmware_admin_cluster;
pub use vmware_admin_cluster::Vmware_admin_cluster;
pub mod bare_metal_admin_cluster;
pub use bare_metal_admin_cluster::Bare_metal_admin_cluster;
pub mod bare_metal_node_pool;
pub use bare_metal_node_pool::Bare_metal_node_pool;
pub mod vmware_node_pool;
pub use vmware_node_pool::Vmware_node_pool;
pub mod location;
pub use location::Location;
pub mod operation;
pub use operation::Operation;
pub mod bare_metal_cluster;
pub use bare_metal_cluster::Bare_metal_cluster;

