//! Core traits and types for pptx-rs
//!
//! This module provides the foundational traits that enable trait-based
//! XML generation and consistent behavior across all PPTX elements.

mod dimension;
mod traits;
mod xml_utils;

pub use dimension::{Dimension, FlexPosition, FlexSize, SLIDE_HEIGHT_EMU, SLIDE_WIDTH_EMU};
pub use traits::{Positioned, Sized as ElementSized, ToXml};
pub use xml_utils::{escape_xml, XmlWriter};
