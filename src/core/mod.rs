//! Core traits and types for pptx-rs
//!
//! This module provides the foundational traits that enable trait-based
//! XML generation and consistent behavior across all PPTX elements.

mod dimension;
mod placement;
mod traits;
mod validation;
mod xml_utils;

pub use dimension::{Dimension, FlexPosition, FlexSize, SLIDE_HEIGHT_EMU, SLIDE_WIDTH_EMU};
pub use placement::ElementPlacement;
pub use traits::{Positioned, Sized as ElementSized, ToXml};
pub use validation::{
    check_required_parts, check_required_parts_with_descriptions, clamp_ratio,
    clamp_unit_interval, validate_index, validate_non_empty, validate_non_empty_str,
    validate_positive, validate_well_formed_xml, RequiredPart, ValidationIssue,
    REQUIRED_PARTS_MINIMAL, REQUIRED_PARTS_REPAIR,
};
pub use xml_utils::{append_i32, append_usize, escape_xml, XmlWriter};
