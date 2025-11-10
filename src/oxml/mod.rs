//! OpenXML processing module
//!
//! Handles XML parsing and generation for OpenXML elements

pub mod parser;
pub mod writer;
pub mod ns;
pub mod traits;
pub mod builder;
pub mod streaming;

pub use parser::parse_xml;
pub use writer::serialize_xml;
pub use ns::*;
pub use traits::{
    OpenXmlElementInfo, OpenXmlElementType, OpenXmlLeafElement, OpenXmlNodeElement,
    OpenXmlRootElement, OpenXmlSerialize, OpenXmlDeserialize,
};
pub use builder::{XmlBuilder, generate_slide_xml, generate_presentation_xml};
pub use streaming::{StreamingXmlReader, StreamingXmlWriter, XmlEvent};

