//! Resource modules

pub mod location;
pub use location::Location;
pub mod flex_template;
pub use flex_template::Flex_template;
pub mod template;
pub use template::Template;
pub mod message;
pub use message::Message;
pub mod job;
pub use job::Job;
pub mod project;
pub use project::Project;
pub mod work_item;
pub use work_item::Work_item;
pub mod stage;
pub use stage::Stage;
pub mod debug;
pub use debug::Debug;
pub mod snapshot;
pub use snapshot::Snapshot;

