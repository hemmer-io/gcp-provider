//! Resource modules

pub mod location;
pub use location::Location;
pub mod accelerator_type;
pub use accelerator_type::Accelerator_type;
pub mod queued_resource;
pub use queued_resource::Queued_resource;
pub mod runtime_version;
pub use runtime_version::Runtime_version;
pub mod node;
pub use node::Node;
pub mod operation;
pub use operation::Operation;
pub mod reservation;
pub use reservation::Reservation;

