//! Resource modules

pub mod version;
pub use version::Version;
pub mod spec;
pub use spec::Spec;
pub mod location;
pub use location::Location;
pub mod deployment;
pub use deployment::Deployment;
pub mod runtime;
pub use runtime::Runtime;
pub mod api;
pub use api::Api;
pub mod document;
pub use document::Document;
pub mod instance;
pub use instance::Instance;
pub mod operation;
pub use operation::Operation;
pub mod artifact;
pub use artifact::Artifact;

