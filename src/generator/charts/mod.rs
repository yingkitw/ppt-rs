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
mod embedding;
pub mod xml;

pub use types::ChartType;
pub use data::{Chart, ChartSeries};
pub use builder::ChartBuilder;
pub use embedding::{
    chart_embedding_filename, create_chart_rels_xml, reference_workbook_bytes,
};
pub use xml::{generate_chart_part_xml, generate_chart_ref_xml};
pub(crate) use crate::core::escape_xml;
