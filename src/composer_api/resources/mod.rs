//! Resource modules

pub mod user_workloads_secret;
pub use user_workloads_secret::User_workloads_secret;
pub mod operation;
pub use operation::Operation;
pub mod workload;
pub use workload::Workload;
pub mod environment;
pub use environment::Environment;
pub mod image_version;
pub use image_version::Image_version;
pub mod user_workloads_config_map;
pub use user_workloads_config_map::User_workloads_config_map;

