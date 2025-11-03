//! Resource modules

pub mod dpc;
pub use dpc::Dpc;
pub mod customer;
pub use customer::Customer;
pub mod device;
pub use device::Device;
pub mod operation;
pub use operation::Operation;
pub mod configuration;
pub use configuration::Configuration;
pub mod vendor;
pub use vendor::Vendor;

