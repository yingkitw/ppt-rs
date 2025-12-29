//! Chart creation support for PPTX generation
//!
//! Modular chart system:
//! - `types` - Chart type definitions
//! - `data` - Chart data structures (Series, Chart)
//! - `builder` - Fluent chart builder
//! - `xml` - XML generation for charts

mod types;
mod data;
mod builder;
pub mod xml;
pub mod excel;
pub mod relationships;

pub use types::ChartType;
pub use data::{Chart, ChartSeries};
pub use builder::ChartBuilder;
pub use xml::{generate_chart_xml, generate_chart_xml_with_number};
pub use relationships::*;

/// Escape XML special characters
pub(crate) fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
