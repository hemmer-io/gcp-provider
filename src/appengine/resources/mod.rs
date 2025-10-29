//! Resource modules

pub mod authorized_domain;
pub use authorized_domain::Authorized_domain;
pub mod operation;
pub use operation::Operation;
pub mod instance;
pub use instance::Instance;
pub mod authorized_certificate;
pub use authorized_certificate::Authorized_certificate;
pub mod service;
pub use service::Service;
pub mod app;
pub use app::App;
pub mod location;
pub use location::Location;
pub mod domain_mapping;
pub use domain_mapping::Domain_mapping;
pub mod ingress_rule;
pub use ingress_rule::Ingress_rule;
pub mod application;
pub use application::Application;
pub mod version;
pub use version::Version;

