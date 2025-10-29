//! Resource modules

pub mod object;
pub use object::Object;
pub mod bucket;
pub use bucket::Bucket;
pub mod bucket_access_control;
pub use bucket_access_control::Bucket_access_control;
pub mod object_access_control;
pub use object_access_control::Object_access_control;

