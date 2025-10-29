//! Resource modules

pub mod backup_vault;
pub use backup_vault::Backup_vault;
pub mod storage_pool;
pub use storage_pool::Storage_pool;
pub mod active_directorie;
pub use active_directorie::Active_directorie;
pub mod volume;
pub use volume::Volume;
pub mod quota_rule;
pub use quota_rule::Quota_rule;
pub mod snapshot;
pub use snapshot::Snapshot;
pub mod location;
pub use location::Location;
pub mod backup;
pub use backup::Backup;
pub mod kms_config;
pub use kms_config::Kms_config;
pub mod replication;
pub use replication::Replication;
pub mod operation;
pub use operation::Operation;
pub mod backup_policie;
pub use backup_policie::Backup_policie;

