//! Resource modules

pub mod customer;
pub use customer::Customer;
pub mod configuration;
pub use configuration::Configuration;
pub mod operation;
pub use operation::Operation;
pub mod dpc;
pub use dpc::Dpc;
pub mod vendor;
pub use vendor::Vendor;
pub mod device;
pub use device::Device;

