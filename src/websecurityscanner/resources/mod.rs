//! Resource modules

pub mod crawled_url;
pub use crawled_url::Crawled_url;
pub mod finding_type_stat;
pub use finding_type_stat::Finding_type_stat;
pub mod finding;
pub use finding::Finding;
pub mod scan_run;
pub use scan_run::Scan_run;
pub mod scan_config;
pub use scan_config::Scan_config;

