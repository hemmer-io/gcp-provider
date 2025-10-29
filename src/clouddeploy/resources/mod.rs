//! Resource modules

pub mod target;
pub use target::Target;
pub mod deploy_policie;
pub use deploy_policie::Deploy_policie;
pub mod rollout;
pub use rollout::Rollout;
pub mod release;
pub use release::Release;
pub mod custom_target_type;
pub use custom_target_type::Custom_target_type;
pub mod automation_run;
pub use automation_run::Automation_run;
pub mod delivery_pipeline;
pub use delivery_pipeline::Delivery_pipeline;
pub mod job_run;
pub use job_run::Job_run;
pub mod location;
pub use location::Location;
pub mod automation;
pub use automation::Automation;
pub mod operation;
pub use operation::Operation;

