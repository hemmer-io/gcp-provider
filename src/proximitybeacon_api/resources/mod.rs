//! Resource modules

pub mod namespace;
pub use namespace::Namespace;
pub mod beacon;
pub use beacon::Beacon;
pub mod attachment;
pub use attachment::Attachment;
pub mod proximitybeacon;
pub use proximitybeacon::Proximitybeacon;
pub mod beaconinfo;
pub use beaconinfo::Beaconinfo;
pub mod diagnostic;
pub use diagnostic::Diagnostic;

