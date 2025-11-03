//! Resource modules

pub mod operation;
pub use operation::Operation;
pub mod catalog_item;
pub use catalog_item::Catalog_item;
pub mod placement;
pub use placement::Placement;
pub mod catalog;
pub use catalog::Catalog;
pub mod prediction_api_key_registration;
pub use prediction_api_key_registration::Prediction_api_key_registration;
pub mod user_event;
pub use user_event::User_event;

