//! Main presentation module

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
pub mod master_customization;
pub mod initialization;

pub use presentation::{Presentation, open_from_package, slide_width, set_slide_width, slide_height, set_slide_height};
pub use relationships::PresentationRelationshipManager;
pub use protection::{DocumentProtection, EditingRestriction};
pub use slide_layouts_collection::{SlideLayoutsCollection, SlideLayoutInfo};
pub use slide_master::{SlideMaster, SlideMasters, SlideMasterInfo};
pub use sections::{Section, SectionCollection};
pub use footer::FooterHeader;
pub use traits::{Dimensioned, PropertyAccessor, Saveable, Openable, Metadata, Collection, CollectionBase};
pub use properties::PropertiesManager;
pub use master_customization::{MasterSlideCustomization, CustomLayout, LayoutType, PlaceholderPosition, PlaceholderStyle};
pub use initialization::{InitContext, initialize_presentation};

