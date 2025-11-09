//! Main presentation module

mod dimensions;
mod open;
mod save;
mod presentation;
mod relationships;
pub mod protection;

pub use presentation::Presentation;
pub use relationships::PresentationRelationshipManager;
pub use protection::{DocumentProtection, EditingRestriction};

