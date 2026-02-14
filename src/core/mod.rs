//! Core traits and types for pptx-rs
//!
//! This module provides the foundational traits that enable trait-based
//! XML generation and consistent behavior across all PPTX elements.

mod traits;
mod xml_utils;
mod dimension;

pub use traits::{ToXml, XmlElement, Positioned, Sized as ElementSized, Styled};
pub use xml_utils::{escape_xml, XmlWriter};
pub use dimension::{Dimension, FlexPosition, FlexSize, SLIDE_WIDTH_EMU, SLIDE_HEIGHT_EMU};
