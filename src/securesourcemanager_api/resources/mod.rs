//! Resource modules

pub mod instance;
pub use instance::Instance;
pub mod issue;
pub use issue::Issue;
pub mod issue_comment;
pub use issue_comment::Issue_comment;
pub mod operation;
pub use operation::Operation;
pub mod pull_request;
pub use pull_request::Pull_request;
pub mod location;
pub use location::Location;
pub mod pull_request_comment;
pub use pull_request_comment::Pull_request_comment;
pub mod branch_rule;
pub use branch_rule::Branch_rule;
pub mod repositorie;
pub use repositorie::Repositorie;
pub mod hook;
pub use hook::Hook;

