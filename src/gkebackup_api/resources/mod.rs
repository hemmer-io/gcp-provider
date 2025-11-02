//! Resource modules

pub mod volume_backup;
pub use volume_backup::Volume_backup;
pub mod location;
pub use location::Location;
pub mod restore_plan;
pub use restore_plan::Restore_plan;
pub mod backup_plan;
pub use backup_plan::Backup_plan;
pub mod backup_plan_binding;
pub use backup_plan_binding::Backup_plan_binding;
pub mod restore;
pub use restore::Restore;
pub mod backup_channel;
pub use backup_channel::Backup_channel;
pub mod volume_restore;
pub use volume_restore::Volume_restore;
pub mod restore_channel;
pub use restore_channel::Restore_channel;
pub mod operation;
pub use operation::Operation;
pub mod backup;
pub use backup::Backup;
pub mod restore_plan_binding;
pub use restore_plan_binding::Restore_plan_binding;

