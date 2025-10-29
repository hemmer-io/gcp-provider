//! Resource modules

pub mod build;
pub use build::Build;
pub mod location;
pub use location::Location;
pub mod backend;
pub use backend::Backend;
pub mod rollout;
pub use rollout::Rollout;
pub mod operation;
pub use operation::Operation;
pub mod domain;
pub use domain::Domain;
pub mod traffic;
pub use traffic::Traffic;

