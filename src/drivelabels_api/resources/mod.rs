//! Resource modules

pub mod user;
pub use user::User;
pub mod label;
pub use label::Label;
pub mod permission;
pub use permission::Permission;
pub mod limit;
pub use limit::Limit;
pub mod lock;
pub use lock::Lock;
pub mod revision;
pub use revision::Revision;

