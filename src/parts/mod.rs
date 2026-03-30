//! Package parts module

pub mod app_props;
pub mod base;
pub mod chart;
pub mod content_types;
pub mod coreprops;
pub mod image;
pub mod media;
pub mod notes_slide;
pub mod presentation;
pub mod relationships;
pub mod slide;
pub mod slide_layout;
pub mod slide_master;
pub mod table;
pub mod theme;

pub use app_props::AppPropertiesPart;
pub use base::{ContentType, Part, PartType};
pub use chart::ChartPart;
pub use content_types::{ContentTypesPart, DefaultType, OverrideType};
pub use coreprops::CorePropertiesPart;
pub use image::ImagePart;
pub use media::{MediaFormat, MediaPart};
pub use notes_slide::NotesSlidePart;
pub use presentation::PresentationPart;
pub use relationships::{Relationship, RelationshipType, Relationships};
pub use slide::SlidePart;
pub use slide_layout::{LayoutType, SlideLayoutPart};
pub use slide_master::SlideMasterPart;
pub use table::{
    BorderStyle, CellBorder, CellBorders, CellMargins, HorizontalAlign, TableCellPart, TablePart,
    TableRowPart, VerticalAlign,
};
pub use theme::{ThemeColor, ThemeFont, ThemePart};
