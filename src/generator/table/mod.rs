//! Table module for PPTX generation
//!
//! This module provides comprehensive table support including:
//! - Cell content and formatting
//! - Row and column management
//! - Text alignment (horizontal and vertical)
//! - Cell backgrounds and borders
//! - Text wrapping
//! - XML generation for OOXML

mod cell;
mod row;
mod builder;
mod format;
mod style;
mod xml;

pub use cell::{TableCell, CellAlign, CellVAlign};
pub use row::TableRow;
pub use builder::{Table, TableBuilder};
pub use format::generate_cell_xml;
pub use style::{header_cell, table_from_string_rows, IMPORT_HEADER_BG, HELPER_HEADER_BG, HEADER_TEXT};
pub use xml::generate_table_xml;
