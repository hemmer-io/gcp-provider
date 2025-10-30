//! Resource modules

pub mod location;
pub use location::Location;
pub mod rollout;
pub use rollout::Rollout;
pub mod domain;
pub use domain::Domain;
pub mod backend;
pub use backend::Backend;
pub mod traffic;
pub use traffic::Traffic;
pub mod operation;
pub use operation::Operation;
pub mod build;
pub use build::Build;

