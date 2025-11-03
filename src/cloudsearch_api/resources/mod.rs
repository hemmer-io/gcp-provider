//! Resource modules

pub mod stat;
pub use stat::Stat;
pub mod setting;
pub use setting::Setting;
pub mod lro;
pub use lro::Lro;
pub mod item;
pub use item::Item;
pub mod datasource;
pub use datasource::Datasource;
pub mod searchapplication;
pub use searchapplication::Searchapplication;
pub mod query;
pub use query::Query;
pub mod unmappedid;
pub use unmappedid::Unmappedid;
pub mod media;
pub use media::Media;
pub mod cloudsearch;
pub use cloudsearch::Cloudsearch;
pub mod source;
pub use source::Source;
pub mod operation;
pub use operation::Operation;

