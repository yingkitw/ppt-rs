//! Presentation initialization helpers
//!
//! Centralizes common initialization logic for presentations

use crate::error::Result;
use crate::opc::{CoreProperties, AppProperties, CustomProperties};
use crate::parts::presentation::PresentationPart;

/// Presentation initialization context
#[derive(Clone, Debug)]
pub struct InitContext {
    /// Core properties
    pub core_properties: CoreProperties,
    /// App properties
    pub app_properties: AppProperties,
    /// Custom properties
    pub custom_properties: CustomProperties,
}

impl InitContext {
    /// Create a new initialization context with default properties
    pub fn new() -> Self {
        Self {
            core_properties: CoreProperties::new(),
            app_properties: AppProperties::new(),
            custom_properties: CustomProperties::new(),
        }
    }

    /// Create initialization context from existing properties
    pub fn from_properties(
        core: CoreProperties,
        app: AppProperties,
        custom: CustomProperties,
    ) -> Self {
        Self {
            core_properties: core,
            app_properties: app,
            custom_properties: custom,
        }
    }
}

impl Default for InitContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to initialize a presentation with standard components
pub fn initialize_presentation(
    part: PresentationPart,
    package: crate::opc::package::Package,
) -> Result<(
    PresentationPart,
    crate::opc::package::Package,
    crate::presentation::SlideLayoutsCollection,
    crate::presentation::SlideMasters,
    InitContext,
)> {
    use crate::presentation::{SlideLayoutsCollection, SlideMasters};

    let slide_layouts = SlideLayoutsCollection::new();
    let slide_masters = SlideMasters::new();
    let init_context = InitContext::new();

    Ok((part, package, slide_layouts, slide_masters, init_context))
}
