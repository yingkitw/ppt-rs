//! XML generation for PPTX components
//!
//! This module re-exports from submodules for backward compatibility.

// Re-export from submodules
pub use super::slide_content::{SlideLayout, SlideContent};
pub use super::package_xml::{
    escape_xml,
    create_content_types_xml,
    create_rels_xml,
    create_presentation_rels_xml,
    create_presentation_xml,
};
pub use super::slide_xml::{
    create_slide_xml,
    create_slide_xml_with_content,
    create_slide_rels_xml,
};
pub use super::theme_xml::{
    create_slide_layout_xml,
    create_slide_layout_xml_by_type,
    create_layout_rels_xml,
    create_layout_rels_xml_for_layout,
    create_slide_master_xml,
    create_master_rels_xml,
    create_theme_xml,
};
pub use super::props_xml::{
    create_core_props_xml,
    create_app_props_xml,
};
