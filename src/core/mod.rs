//! Core traits and types for pptx-rs
//!
//! This module provides the foundational traits that enable trait-based
//! XML generation and consistent behavior across all PPTX elements.

mod dimension;
mod package_validation;
mod placement;
mod powerpoint_compat;
mod traits;
mod validation;
mod xml_utils;

pub use dimension::{Dimension, FlexPosition, FlexSize, SLIDE_HEIGHT_EMU, SLIDE_WIDTH_EMU};
pub use package_validation::{
    validate_package, validate_package_bytes, PackageValidationIssue, PackageValidationReport,
    REQUIRED_PACKAGE_PARTS, ValidationCategory, ValidationSeverity,
};
pub use placement::ElementPlacement;
pub use powerpoint_compat::{validate_powerpoint_structure, CompatReport};
pub use traits::{Positioned, Sized as ElementSized, ToXml};
pub use validation::{
    check_required_parts, check_required_parts_with_descriptions, clamp_ratio,
    clamp_unit_interval, validate_index, validate_non_empty, validate_non_empty_str,
    validate_positive, validate_well_formed_xml, RequiredPart, ValidationIssue,
    REQUIRED_PARTS_MINIMAL, REQUIRED_PARTS_REPAIR,
};
pub use xml_utils::{append_escape_xml, append_i32, append_usize, escape_xml, XmlWriter};
