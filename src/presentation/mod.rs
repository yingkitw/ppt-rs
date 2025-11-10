//! Main presentation module

mod dimensions;
mod open;
mod save;
mod presentation;
mod relationships;
pub mod protection;
mod slide_layouts_collection;
mod slide_master;
pub mod sections;
pub mod footer;
pub mod traits;
pub mod properties;

pub use presentation::Presentation;
pub use relationships::PresentationRelationshipManager;
pub use protection::{DocumentProtection, EditingRestriction};
pub use slide_layouts_collection::{SlideLayoutsCollection, SlideLayoutInfo};
pub use slide_master::{SlideMaster, SlideMasters, SlideMasterInfo};
pub use sections::{Section, SectionCollection};
pub use footer::FooterHeader;
pub use traits::{Dimensioned, PropertyAccessor, Saveable, Openable, Metadata, Collection};
pub use properties::PropertiesManager;

