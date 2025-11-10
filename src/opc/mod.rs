//! Open Packaging Convention (OPC) module
//!
//! Handles the ZIP-based package structure of .pptx files

pub mod package;
pub mod part;
pub mod constants;
pub mod packuri;
pub mod relationships;
pub mod serialized;
pub mod spec;
pub mod content_types;
pub mod namespace;
pub mod properties_enhanced;
pub mod document;
pub mod part_container;

pub use package::Package;
pub use part::Part;
pub use constants::*;
pub use packuri::PackURI;
pub use serialized::{PackageReader, PackageWriter};
pub use content_types::ContentTypesManager;
pub use namespace::Namespaces;
pub use properties_enhanced::{CoreProperties, AppProperties, CustomProperties};
pub use document::{OpenXmlDocument, DocumentFormat};
pub use part_container::PartContainer;

