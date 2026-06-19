//! Embedded Excel workbook for chart data (PowerPoint requires package link).

/// Filename for the embedded workbook associated with a chart part.
pub fn chart_embedding_filename(chart_idx: usize) -> String {
    format!("Microsoft_Excel_Sheet{chart_idx}.xlsx")
}

/// Relationship file linking a chart part to its embedded workbook.
pub fn create_chart_rels_xml(embedding_filename: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" Target="../embeddings/{embedding_filename}"/>
</Relationships>"#
    )
}

/// Reference workbook bytes extracted from a PowerPoint-compatible chart deck.
pub fn reference_workbook_bytes() -> &'static [u8] {
    include_bytes!("../reference_chart_embedding.xlsx")
}
