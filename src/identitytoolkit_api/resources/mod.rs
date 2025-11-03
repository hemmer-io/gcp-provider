//! Resource modules

pub mod default_supported_idp_config;
pub use default_supported_idp_config::Default_supported_idp_config;
pub mod account;
pub use account::Account;
pub mod identity_platform;
pub use identity_platform::Identity_platform;
pub mod default_supported_idp;
pub use default_supported_idp::Default_supported_idp;
pub mod oauth_idp_config;
pub use oauth_idp_config::Oauth_idp_config;
pub mod identitytoolkit;
pub use identitytoolkit::Identitytoolkit;
pub mod inbound_saml_config;
pub use inbound_saml_config::Inbound_saml_config;
pub mod mfa_sign_in;
pub use mfa_sign_in::Mfa_sign_in;
pub mod mfa_enrollment;
pub use mfa_enrollment::Mfa_enrollment;
pub mod project;
pub use project::Project;
pub mod tenant;
pub use tenant::Tenant;
pub mod relyingparty;
pub use relyingparty::Relyingparty;

