//! OXML (Office XML) element handling

pub mod editor;
pub mod ns;
pub mod presentation;
pub mod repair;
pub mod slide;
pub mod xmlchemy;

pub use editor::PresentationEditor;
pub use ns::Namespace;
pub use presentation::{PresentationInfo, PresentationReader};
pub use repair::{PptxRepair, RepairIssue, RepairResult};
pub use slide::{ParsedSlide, SlideParser};
pub use xmlchemy::{XmlElement, XmlParser};
