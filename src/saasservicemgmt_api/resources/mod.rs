//! Resource modules

pub mod unit_kind;
pub use unit_kind::Unit_kind;
pub mod release;
pub use release::Release;
pub mod tenant;
pub use tenant::Tenant;
pub mod unit;
pub use unit::Unit;
pub mod rollout_kind;
pub use rollout_kind::Rollout_kind;
pub mod rollout;
pub use rollout::Rollout;
pub mod replications_internal;
pub use replications_internal::Replications_internal;
pub mod unit_operation;
pub use unit_operation::Unit_operation;
pub mod location;
pub use location::Location;
pub mod saa;
pub use saa::Saa;

