//! Resource modules

pub mod observation_job;
pub use observation_job::Observation_job;
pub mod operation;
pub use operation::Operation;
pub mod location;
pub use location::Location;
pub mod api_observation;
pub use api_observation::Api_observation;
pub mod api_operation;
pub use api_operation::Api_operation;
pub mod observation_source;
pub use observation_source::Observation_source;

