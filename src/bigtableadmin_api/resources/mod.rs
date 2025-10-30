//! Resource modules

pub mod location;
pub use location::Location;
pub mod operation;
pub use operation::Operation;
pub mod backup;
pub use backup::Backup;
pub mod authorized_view;
pub use authorized_view::Authorized_view;
pub mod instance;
pub use instance::Instance;
pub mod cluster;
pub use cluster::Cluster;
pub mod hot_tablet;
pub use hot_tablet::Hot_tablet;
pub mod schema_bundle;
pub use schema_bundle::Schema_bundle;
pub mod logical_view;
pub use logical_view::Logical_view;
pub mod table;
pub use table::Table;
pub mod app_profile;
pub use app_profile::App_profile;
pub mod materialized_view;
pub use materialized_view::Materialized_view;

