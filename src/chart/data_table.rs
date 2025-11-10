//! Chart Data Table - Display data table in chart
//! 
//! Implements ECMA-376 standard data table for charts with all display options.

/// Data table display options (ECMA-376 compliant)
#[derive(Clone, Debug)]
pub struct DataTable {
    /// Show legend keys in data table
    show_legend_keys: bool,
    /// Show values
    show_val: bool,
    /// Show category names
    show_cat_name: bool,
    /// Show series names
    show_ser_name: bool,
    /// Show percentages
    show_percent: bool,
    /// Show bubble size
    show_bubble_size: bool,
    /// Show horizontal border
    show_h_border: bool,
    /// Show vertical border
    show_v_border: bool,
    /// Show outline
    show_outline: bool,
}

impl DataTable {
    /// Create a new data table with defaults (all false except borders/outline)
    pub fn new() -> Self {
        Self {
            show_legend_keys: false,
            show_val: false,
            show_cat_name: false,
            show_ser_name: false,
            show_percent: false,
            show_bubble_size: false,
            show_h_border: true,
            show_v_border: true,
            show_outline: true,
        }
    }

    /// Set show legend keys (fluent)
    pub fn show_legend_keys(mut self, show: bool) -> Self {
        self.show_legend_keys = show;
        self
    }

    /// Get show legend keys
    pub fn has_legend_keys(&self) -> bool {
        self.show_legend_keys
    }

    /// Set show values (fluent)
    pub fn show_values(mut self, show: bool) -> Self {
        self.show_val = show;
        self
    }

    /// Get show values
    pub fn has_values(&self) -> bool {
        self.show_val
    }

    /// Set show category names (fluent)
    pub fn show_category_names(mut self, show: bool) -> Self {
        self.show_cat_name = show;
        self
    }

    /// Get show category names
    pub fn has_category_names(&self) -> bool {
        self.show_cat_name
    }

    /// Set show series names (fluent)
    pub fn show_series_names(mut self, show: bool) -> Self {
        self.show_ser_name = show;
        self
    }

    /// Get show series names
    pub fn has_series_names(&self) -> bool {
        self.show_ser_name
    }

    /// Set show percentages (fluent)
    pub fn show_percentages(mut self, show: bool) -> Self {
        self.show_percent = show;
        self
    }

    /// Get show percentages
    pub fn has_percentages(&self) -> bool {
        self.show_percent
    }

    /// Set show bubble size (fluent)
    pub fn show_bubble_size(mut self, show: bool) -> Self {
        self.show_bubble_size = show;
        self
    }

    /// Get show bubble size
    pub fn has_bubble_size(&self) -> bool {
        self.show_bubble_size
    }

    /// Set show horizontal border (fluent)
    pub fn show_h_border(mut self, show: bool) -> Self {
        self.show_h_border = show;
        self
    }

    /// Get show horizontal border
    pub fn has_h_border(&self) -> bool {
        self.show_h_border
    }

    /// Set show vertical border (fluent)
    pub fn show_v_border(mut self, show: bool) -> Self {
        self.show_v_border = show;
        self
    }

    /// Get show vertical border
    pub fn has_v_border(&self) -> bool {
        self.show_v_border
    }

    /// Set show outline (fluent)
    pub fn show_outline(mut self, show: bool) -> Self {
        self.show_outline = show;
        self
    }

    /// Get show outline
    pub fn has_outline(&self) -> bool {
        self.show_outline
    }

    /// Generate XML for data table (ECMA-376 compliant)
    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<c:dTable>");
        
        // Always include all elements in order per ECMA-376
        xml.push_str(&format!("<c:showLegendKey val=\"{}\"/>", if self.show_legend_keys { "1" } else { "0" }));
        xml.push_str(&format!("<c:showVal val=\"{}\"/>", if self.show_val { "1" } else { "0" }));
        xml.push_str(&format!("<c:showCatName val=\"{}\"/>", if self.show_cat_name { "1" } else { "0" }));
        xml.push_str(&format!("<c:showSerName val=\"{}\"/>", if self.show_ser_name { "1" } else { "0" }));
        xml.push_str(&format!("<c:showPercent val=\"{}\"/>", if self.show_percent { "1" } else { "0" }));
        xml.push_str(&format!("<c:showBubbleSize val=\"{}\"/>", if self.show_bubble_size { "1" } else { "0" }));
        xml.push_str(&format!("<c:showHBorder val=\"{}\"/>", if self.show_h_border { "1" } else { "0" }));
        xml.push_str(&format!("<c:showVBorder val=\"{}\"/>", if self.show_v_border { "1" } else { "0" }));
        xml.push_str(&format!("<c:showOutline val=\"{}\"/>", if self.show_outline { "1" } else { "0" }));
        
        xml.push_str("</c:dTable>");
        xml
    }
}

impl Default for DataTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_table_creation() {
        let table = DataTable::new();
        assert!(!table.has_legend_keys());
        assert!(table.has_h_border());
        assert!(table.has_v_border());
        assert!(table.has_outline());
    }

    #[test]
    fn test_data_table_fluent_api() {
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_category_names(true)
            .show_h_border(false);
        
        assert!(table.has_legend_keys());
        assert!(table.has_values());
        assert!(table.has_category_names());
        assert!(!table.has_h_border());
    }

    #[test]
    fn test_data_table_all_options() {
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_category_names(true)
            .show_series_names(true)
            .show_percentages(true)
            .show_bubble_size(true);
        
        assert!(table.has_legend_keys());
        assert!(table.has_values());
        assert!(table.has_category_names());
        assert!(table.has_series_names());
        assert!(table.has_percentages());
        assert!(table.has_bubble_size());
    }

    #[test]
    fn test_data_table_xml_generation() {
        let table = DataTable::new();
        let xml = table.to_xml();
        assert!(xml.contains("<c:dTable>"));
        assert!(xml.contains("<c:showLegendKey val=\"0\"/>"));
        assert!(xml.contains("<c:showHBorder val=\"1\"/>"));
        assert!(xml.contains("</c:dTable>"));
    }

    #[test]
    fn test_data_table_xml_all_enabled() {
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_category_names(true)
            .show_series_names(true)
            .show_percentages(true)
            .show_bubble_size(true);
        
        let xml = table.to_xml();
        assert!(xml.contains("<c:showLegendKey val=\"1\"/>"));
        assert!(xml.contains("<c:showVal val=\"1\"/>"));
        assert!(xml.contains("<c:showCatName val=\"1\"/>"));
        assert!(xml.contains("<c:showSerName val=\"1\"/>"));
        assert!(xml.contains("<c:showPercent val=\"1\"/>"));
        assert!(xml.contains("<c:showBubbleSize val=\"1\"/>"));
    }

    #[test]
    fn test_data_table_xml_all_disabled() {
        let table = DataTable::new()
            .show_legend_keys(false)
            .show_values(false)
            .show_category_names(false)
            .show_series_names(false)
            .show_percentages(false)
            .show_bubble_size(false)
            .show_h_border(false)
            .show_v_border(false)
            .show_outline(false);
        
        let xml = table.to_xml();
        assert!(xml.contains("<c:showLegendKey val=\"0\"/>"));
        assert!(xml.contains("<c:showHBorder val=\"0\"/>"));
        assert!(xml.contains("<c:showVBorder val=\"0\"/>"));
        assert!(xml.contains("<c:showOutline val=\"0\"/>"));
    }

    #[test]
    fn test_data_table_default() {
        let table = DataTable::default();
        assert_eq!(table.to_xml(), DataTable::new().to_xml());
    }

    // ========== PYTHON-PPTX REFERENCE COMPARISON TESTS ==========
    
    #[test]
    fn test_data_table_xml_structure_matches_ecma376() {
        // ECMA-376 standard requires specific element order
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_category_names(true)
            .show_series_names(true)
            .show_percentages(true)
            .show_bubble_size(true)
            .show_h_border(true)
            .show_v_border(true)
            .show_outline(true);
        
        let xml = table.to_xml();
        
        // Verify element order (critical for ECMA-376 compliance)
        let legend_pos = xml.find("<c:showLegendKey").unwrap();
        let val_pos = xml.find("<c:showVal").unwrap();
        let cat_pos = xml.find("<c:showCatName").unwrap();
        let ser_pos = xml.find("<c:showSerName").unwrap();
        let pct_pos = xml.find("<c:showPercent").unwrap();
        let bubble_pos = xml.find("<c:showBubbleSize").unwrap();
        let hborder_pos = xml.find("<c:showHBorder").unwrap();
        let vborder_pos = xml.find("<c:showVBorder").unwrap();
        let outline_pos = xml.find("<c:showOutline").unwrap();
        
        // Elements must appear in this exact order
        assert!(legend_pos < val_pos);
        assert!(val_pos < cat_pos);
        assert!(cat_pos < ser_pos);
        assert!(ser_pos < pct_pos);
        assert!(pct_pos < bubble_pos);
        assert!(bubble_pos < hborder_pos);
        assert!(hborder_pos < vborder_pos);
        assert!(vborder_pos < outline_pos);
    }

    #[test]
    fn test_data_table_xml_attribute_format() {
        // Verify XML attributes use correct format (val="0" or val="1")
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(false);
        
        let xml = table.to_xml();
        
        assert!(xml.contains(r#"<c:showLegendKey val="1"/>"#));
        assert!(xml.contains(r#"<c:showVal val="0"/>"#));
    }

    #[test]
    fn test_data_table_xml_completeness() {
        // All 9 elements must be present in output
        let table = DataTable::new();
        let xml = table.to_xml();
        
        assert!(xml.contains("<c:showLegendKey"));
        assert!(xml.contains("<c:showVal"));
        assert!(xml.contains("<c:showCatName"));
        assert!(xml.contains("<c:showSerName"));
        assert!(xml.contains("<c:showPercent"));
        assert!(xml.contains("<c:showBubbleSize"));
        assert!(xml.contains("<c:showHBorder"));
        assert!(xml.contains("<c:showVBorder"));
        assert!(xml.contains("<c:showOutline"));
    }

    #[test]
    fn test_data_table_xml_no_extra_elements() {
        // Verify no extra or duplicate elements
        let table = DataTable::new();
        let xml = table.to_xml();
        
        assert_eq!(xml.matches("<c:showLegendKey").count(), 1);
        assert_eq!(xml.matches("<c:showVal").count(), 1);
        assert_eq!(xml.matches("<c:showCatName").count(), 1);
        assert_eq!(xml.matches("<c:showSerName").count(), 1);
        assert_eq!(xml.matches("<c:showPercent").count(), 1);
        assert_eq!(xml.matches("<c:showBubbleSize").count(), 1);
        assert_eq!(xml.matches("<c:showHBorder").count(), 1);
        assert_eq!(xml.matches("<c:showVBorder").count(), 1);
        assert_eq!(xml.matches("<c:showOutline").count(), 1);
    }

    #[test]
    fn test_data_table_default_values() {
        // Default should match python-pptx defaults
        let table = DataTable::new();
        
        // python-pptx defaults: all false except borders/outline
        assert!(!table.has_legend_keys());
        assert!(!table.has_values());
        assert!(!table.has_category_names());
        assert!(!table.has_series_names());
        assert!(!table.has_percentages());
        assert!(!table.has_bubble_size());
        assert!(table.has_h_border());
        assert!(table.has_v_border());
        assert!(table.has_outline());
    }

    #[test]
    fn test_data_table_all_combinations() {
        // Test various combinations to ensure consistency
        let combinations = vec![
            (true, true, true, true, true, true, true, true, true),
            (false, false, false, false, false, false, false, false, false),
            (true, false, true, false, true, false, true, false, true),
            (false, true, false, true, false, true, false, true, false),
        ];
        
        for (lk, val, cn, sn, pct, bs, hb, vb, out) in combinations {
            let table = DataTable::new()
                .show_legend_keys(lk)
                .show_values(val)
                .show_category_names(cn)
                .show_series_names(sn)
                .show_percentages(pct)
                .show_bubble_size(bs)
                .show_h_border(hb)
                .show_v_border(vb)
                .show_outline(out);
            
            let xml = table.to_xml();
            
            // Verify each setting is reflected in XML
            assert_eq!(xml.contains(r#"<c:showLegendKey val="1"/>"#), lk);
            assert_eq!(xml.contains(r#"<c:showVal val="1"/>"#), val);
            assert_eq!(xml.contains(r#"<c:showCatName val="1"/>"#), cn);
            assert_eq!(xml.contains(r#"<c:showSerName val="1"/>"#), sn);
            assert_eq!(xml.contains(r#"<c:showPercent val="1"/>"#), pct);
            assert_eq!(xml.contains(r#"<c:showBubbleSize val="1"/>"#), bs);
            assert_eq!(xml.contains(r#"<c:showHBorder val="1"/>"#), hb);
            assert_eq!(xml.contains(r#"<c:showVBorder val="1"/>"#), vb);
            assert_eq!(xml.contains(r#"<c:showOutline val="1"/>"#), out);
        }
    }

    #[test]
    fn test_data_table_immutability_with_fluent() {
        // Fluent API should not modify original
        let table1 = DataTable::new();
        let table2 = table1.clone().show_legend_keys(true);
        
        assert!(!table1.has_legend_keys());
        assert!(table2.has_legend_keys());
    }

    #[test]
    fn test_data_table_chaining_order() {
        // Chaining order should not matter
        let table1 = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_h_border(false);
        
        let table2 = DataTable::new()
            .show_h_border(false)
            .show_legend_keys(true)
            .show_values(true);
        
        assert_eq!(table1.to_xml(), table2.to_xml());
    }

    #[test]
    fn test_data_table_xml_well_formed() {
        // Verify XML is well-formed
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true);
        
        let xml = table.to_xml();
        
        // Check opening and closing tags match
        assert!(xml.starts_with("<c:dTable>"));
        assert!(xml.ends_with("</c:dTable>"));
        
        // All inner elements should be self-closing
        let self_closing_count = xml.matches("/>").count();
        assert_eq!(self_closing_count, 9); // 9 elements, all self-closing
    }

    #[test]
    fn test_data_table_xml_no_whitespace() {
        // XML should be compact (no unnecessary whitespace)
        let table = DataTable::new();
        let xml = table.to_xml();
        
        // Should not have newlines or extra spaces
        assert!(!xml.contains('\n'));
        assert!(!xml.contains("  ")); // No double spaces
    }

    #[test]
    fn test_data_table_clone_independence() {
        // Cloned tables should be independent
        let mut table1 = DataTable::new();
        let table2 = table1.clone();
        
        table1 = table1.show_legend_keys(true);
        
        assert!(table1.has_legend_keys());
        assert!(!table2.has_legend_keys());
    }

    #[test]
    fn test_data_table_debug_output() {
        // Debug format should be useful
        let table = DataTable::new().show_legend_keys(true);
        let debug_str = format!("{:?}", table);
        
        assert!(debug_str.contains("DataTable"));
        assert!(debug_str.contains("show_legend_keys"));
    }

    #[test]
    fn test_data_table_consistency_across_calls() {
        // Multiple calls to to_xml should produce identical output
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true);
        
        let xml1 = table.to_xml();
        let xml2 = table.to_xml();
        let xml3 = table.to_xml();
        
        assert_eq!(xml1, xml2);
        assert_eq!(xml2, xml3);
    }

    #[test]
    fn test_data_table_edge_case_all_false() {
        // Edge case: all options disabled
        let table = DataTable::new()
            .show_legend_keys(false)
            .show_values(false)
            .show_category_names(false)
            .show_series_names(false)
            .show_percentages(false)
            .show_bubble_size(false)
            .show_h_border(false)
            .show_v_border(false)
            .show_outline(false);
        
        let xml = table.to_xml();
        
        // Should still have all elements with val="0"
        assert_eq!(xml.matches(r#"val="0""#).count(), 9);
        assert_eq!(xml.matches(r#"val="1""#).count(), 0);
    }

    #[test]
    fn test_data_table_edge_case_all_true() {
        // Edge case: all options enabled
        let table = DataTable::new()
            .show_legend_keys(true)
            .show_values(true)
            .show_category_names(true)
            .show_series_names(true)
            .show_percentages(true)
            .show_bubble_size(true)
            .show_h_border(true)
            .show_v_border(true)
            .show_outline(true);
        
        let xml = table.to_xml();
        
        // Should have all elements with val="1"
        assert_eq!(xml.matches(r#"val="1""#).count(), 9);
        assert_eq!(xml.matches(r#"val="0""#).count(), 0);
    }
}
