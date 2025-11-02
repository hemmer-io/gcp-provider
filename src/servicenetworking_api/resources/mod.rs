//! Resource modules

pub mod service;
pub use service::Service;
pub mod connection;
pub use connection::Connection;
pub mod dns_record_set;
pub use dns_record_set::Dns_record_set;
pub mod peered_dns_domain;
pub use peered_dns_domain::Peered_dns_domain;
pub mod operation;
pub use operation::Operation;
pub mod dns_zone;
pub use dns_zone::Dns_zone;
pub mod role;
pub use role::Role;
pub mod network;
pub use network::Network;
pub mod service;
pub use service::Service;
pub mod connection;
pub use connection::Connection;
pub mod operation;
pub use operation::Operation;

