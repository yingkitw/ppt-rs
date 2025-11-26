//! OXML (Office XML) element handling

pub mod action;
pub mod chart;
pub mod coreprops;
pub mod dml;
pub mod ns;
pub mod presentation;
pub mod shapes;
pub mod simpletypes;
pub mod slide;
pub mod table;
pub mod text;
pub mod theme;
pub mod xmlchemy;

pub use ns::Namespace;
pub use xmlchemy::BaseOxmlElement;
