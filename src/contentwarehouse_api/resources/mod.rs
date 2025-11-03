//! Resource modules

pub mod location;
pub use location::Location;
pub mod document;
pub use document::Document;
pub mod reference_id;
pub use reference_id::Reference_id;
pub mod synonym_set;
pub use synonym_set::Synonym_set;
pub mod document_link;
pub use document_link::Document_link;
pub mod document_schema;
pub use document_schema::Document_schema;
pub mod rule_set;
pub use rule_set::Rule_set;
pub mod operation;
pub use operation::Operation;
pub mod project;
pub use project::Project;

