//! Advanced table cell merging
//!
//! Provides utilities for merging cells across rows and columns in tables,
//! generating proper `gridSpan`, `rowSpan`, `vMerge`, and `hMerge` attributes.

/// A merge region defined by top-left corner and span
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergeRegion {
    pub row: usize,
    pub col: usize,
    pub row_span: usize,
    pub col_span: usize,
}

impl MergeRegion {
    /// Create a merge region starting at (row, col) spanning rows × cols
    pub fn new(row: usize, col: usize, row_span: usize, col_span: usize) -> Result<Self, String> {
        if row_span == 0 || col_span == 0 {
            return Err("Span must be at least 1".to_string());
        }
        Ok(Self { row, col, row_span, col_span })
    }

    /// Last row index (inclusive)
    pub fn last_row(&self) -> usize {
        self.row + self.row_span - 1
    }

    /// Last column index (inclusive)
    pub fn last_col(&self) -> usize {
        self.col + self.col_span - 1
    }

    /// Check if a cell (r, c) is inside this merge region
    pub fn contains(&self, r: usize, c: usize) -> bool {
        r >= self.row && r <= self.last_row() && c >= self.col && c <= self.last_col()
    }

    /// Check if this is the anchor cell (top-left)
    pub fn is_anchor(&self, r: usize, c: usize) -> bool {
        r == self.row && c == self.col
    }

    /// Whether this region overlaps with another
    pub fn overlaps(&self, other: &MergeRegion) -> bool {
        self.row <= other.last_row()
            && other.row <= self.last_row()
            && self.col <= other.last_col()
            && other.col <= self.last_col()
    }
}

/// Cell merge state for XML generation
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellMergeState {
    /// Normal cell, not merged
    Normal,
    /// Anchor cell of a merge region
    Anchor { row_span: usize, col_span: usize },
    /// Horizontally merged (covered by col_span of anchor)
    HMerge,
    /// Vertically merged (covered by row_span of anchor)
    VMerge,
}

impl CellMergeState {
    /// Generate XML attributes for `<a:tc>` element
    pub fn to_xml_attrs(&self) -> String {
        match self {
            CellMergeState::Normal => String::new(),
            CellMergeState::Anchor { row_span, col_span } => {
                let mut attrs = String::new();
                if *col_span > 1 {
                    attrs.push_str(&format!(r#" gridSpan="{}""#, col_span));
                }
                if *row_span > 1 {
                    attrs.push_str(&format!(r#" rowSpan="{}""#, row_span));
                }
                attrs
            }
            CellMergeState::HMerge => r#" hMerge="1""#.to_string(),
            CellMergeState::VMerge => r#" vMerge="1""#.to_string(),
        }
    }

    pub fn is_merged_away(&self) -> bool {
        matches!(self, CellMergeState::HMerge | CellMergeState::VMerge)
    }
}

/// Manages merge regions for a table and computes per-cell merge state
#[derive(Clone, Debug, Default)]
pub struct TableMergeMap {
    regions: Vec<MergeRegion>,
    rows: usize,
    cols: usize,
}

impl TableMergeMap {
    /// Create a merge map for a table of given dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { regions: Vec::new(), rows, cols }
    }

    /// Add a merge region. Returns error if out of bounds or overlapping.
    pub fn add_merge(&mut self, region: MergeRegion) -> Result<(), String> {
        if region.last_row() >= self.rows {
            return Err(format!(
                "Merge region row {}-{} exceeds table rows {}",
                region.row, region.last_row(), self.rows
            ));
        }
        if region.last_col() >= self.cols {
            return Err(format!(
                "Merge region col {}-{} exceeds table cols {}",
                region.col, region.last_col(), self.cols
            ));
        }
        for existing in &self.regions {
            if existing.overlaps(&region) {
                return Err(format!(
                    "Merge region ({},{}) {}x{} overlaps with ({},{}) {}x{}",
                    region.row, region.col, region.row_span, region.col_span,
                    existing.row, existing.col, existing.row_span, existing.col_span,
                ));
            }
        }
        self.regions.push(region);
        Ok(())
    }

    /// Merge a rectangular range (convenience method)
    pub fn merge_cells(&mut self, row: usize, col: usize, row_span: usize, col_span: usize) -> Result<(), String> {
        let region = MergeRegion::new(row, col, row_span, col_span)?;
        self.add_merge(region)
    }

    /// Get the merge state for a specific cell
    pub fn cell_state(&self, r: usize, c: usize) -> CellMergeState {
        for region in &self.regions {
            if region.is_anchor(r, c) {
                return CellMergeState::Anchor {
                    row_span: region.row_span,
                    col_span: region.col_span,
                };
            }
            if region.contains(r, c) {
                // Same row as anchor but different col → hMerge
                if r == region.row {
                    return CellMergeState::HMerge;
                }
                // Different row → vMerge
                return CellMergeState::VMerge;
            }
        }
        CellMergeState::Normal
    }

    /// Get all merge regions
    pub fn regions(&self) -> &[MergeRegion] {
        &self.regions
    }

    /// Number of merge regions
    pub fn len(&self) -> usize {
        self.regions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.regions.is_empty()
    }

    /// Table dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_region_new() {
        let r = MergeRegion::new(0, 0, 2, 3).unwrap();
        assert_eq!(r.row, 0);
        assert_eq!(r.col, 0);
        assert_eq!(r.row_span, 2);
        assert_eq!(r.col_span, 3);
    }

    #[test]
    fn test_merge_region_zero_span() {
        assert!(MergeRegion::new(0, 0, 0, 1).is_err());
        assert!(MergeRegion::new(0, 0, 1, 0).is_err());
    }

    #[test]
    fn test_merge_region_last() {
        let r = MergeRegion::new(1, 2, 3, 4).unwrap();
        assert_eq!(r.last_row(), 3);
        assert_eq!(r.last_col(), 5);
    }

    #[test]
    fn test_merge_region_contains() {
        let r = MergeRegion::new(1, 1, 2, 2).unwrap();
        assert!(r.contains(1, 1));
        assert!(r.contains(2, 2));
        assert!(!r.contains(0, 0));
        assert!(!r.contains(3, 1));
    }

    #[test]
    fn test_merge_region_is_anchor() {
        let r = MergeRegion::new(1, 2, 2, 3).unwrap();
        assert!(r.is_anchor(1, 2));
        assert!(!r.is_anchor(1, 3));
        assert!(!r.is_anchor(2, 2));
    }

    #[test]
    fn test_merge_region_overlaps() {
        let a = MergeRegion::new(0, 0, 2, 2).unwrap();
        let b = MergeRegion::new(1, 1, 2, 2).unwrap();
        assert!(a.overlaps(&b));
        assert!(b.overlaps(&a));

        let c = MergeRegion::new(2, 2, 1, 1).unwrap();
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn test_merge_region_adjacent_no_overlap() {
        let a = MergeRegion::new(0, 0, 2, 2).unwrap();
        let b = MergeRegion::new(0, 2, 2, 2).unwrap();
        assert!(!a.overlaps(&b));
    }

    #[test]
    fn test_cell_merge_state_normal() {
        let s = CellMergeState::Normal;
        assert_eq!(s.to_xml_attrs(), "");
        assert!(!s.is_merged_away());
    }

    #[test]
    fn test_cell_merge_state_anchor() {
        let s = CellMergeState::Anchor { row_span: 2, col_span: 3 };
        let xml = s.to_xml_attrs();
        assert!(xml.contains(r#"gridSpan="3""#));
        assert!(xml.contains(r#"rowSpan="2""#));
        assert!(!s.is_merged_away());
    }

    #[test]
    fn test_cell_merge_state_anchor_col_only() {
        let s = CellMergeState::Anchor { row_span: 1, col_span: 3 };
        let xml = s.to_xml_attrs();
        assert!(xml.contains(r#"gridSpan="3""#));
        assert!(!xml.contains("rowSpan"));
    }

    #[test]
    fn test_cell_merge_state_hmerge() {
        let s = CellMergeState::HMerge;
        assert!(s.to_xml_attrs().contains("hMerge"));
        assert!(s.is_merged_away());
    }

    #[test]
    fn test_cell_merge_state_vmerge() {
        let s = CellMergeState::VMerge;
        assert!(s.to_xml_attrs().contains("vMerge"));
        assert!(s.is_merged_away());
    }

    #[test]
    fn test_table_merge_map_new() {
        let m = TableMergeMap::new(5, 4);
        assert!(m.is_empty());
        assert_eq!(m.dimensions(), (5, 4));
    }

    #[test]
    fn test_table_merge_map_add() {
        let mut m = TableMergeMap::new(5, 4);
        assert!(m.merge_cells(0, 0, 2, 2).is_ok());
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn test_table_merge_map_out_of_bounds() {
        let mut m = TableMergeMap::new(3, 3);
        assert!(m.merge_cells(2, 2, 2, 1).is_err()); // row overflow
        assert!(m.merge_cells(0, 2, 1, 2).is_err()); // col overflow
    }

    #[test]
    fn test_table_merge_map_overlap_detection() {
        let mut m = TableMergeMap::new(5, 5);
        m.merge_cells(0, 0, 2, 2).unwrap();
        assert!(m.merge_cells(1, 1, 2, 2).is_err());
    }

    #[test]
    fn test_table_merge_map_adjacent_ok() {
        let mut m = TableMergeMap::new(5, 5);
        m.merge_cells(0, 0, 2, 2).unwrap();
        assert!(m.merge_cells(0, 2, 2, 2).is_ok());
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_table_merge_map_cell_state() {
        let mut m = TableMergeMap::new(4, 4);
        m.merge_cells(0, 0, 2, 3).unwrap();

        // Anchor
        assert_eq!(
            m.cell_state(0, 0),
            CellMergeState::Anchor { row_span: 2, col_span: 3 }
        );
        // hMerge (same row as anchor, different col)
        assert_eq!(m.cell_state(0, 1), CellMergeState::HMerge);
        assert_eq!(m.cell_state(0, 2), CellMergeState::HMerge);
        // vMerge (different row)
        assert_eq!(m.cell_state(1, 0), CellMergeState::VMerge);
        assert_eq!(m.cell_state(1, 1), CellMergeState::VMerge);
        assert_eq!(m.cell_state(1, 2), CellMergeState::VMerge);
        // Normal (outside)
        assert_eq!(m.cell_state(0, 3), CellMergeState::Normal);
        assert_eq!(m.cell_state(2, 0), CellMergeState::Normal);
    }

    #[test]
    fn test_table_merge_map_multiple_regions() {
        let mut m = TableMergeMap::new(4, 4);
        m.merge_cells(0, 0, 1, 2).unwrap();
        m.merge_cells(2, 2, 2, 2).unwrap();

        assert_eq!(
            m.cell_state(0, 0),
            CellMergeState::Anchor { row_span: 1, col_span: 2 }
        );
        assert_eq!(m.cell_state(0, 1), CellMergeState::HMerge);
        assert_eq!(
            m.cell_state(2, 2),
            CellMergeState::Anchor { row_span: 2, col_span: 2 }
        );
        assert_eq!(m.cell_state(3, 3), CellMergeState::VMerge);
        assert_eq!(m.cell_state(1, 1), CellMergeState::Normal);
    }
}
