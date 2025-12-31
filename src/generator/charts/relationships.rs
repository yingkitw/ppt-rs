//! Chart relationship generation
//!
//! Generates chart relationship files that link charts to their Excel data sources

/// Create chart relationship XML that links chart to Excel data source
pub fn create_chart_relationship_xml(_chart_number: usize, excel_filename: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" Target="../embeddings/{}"/>
</Relationships>"#,
        excel_filename
    )
}

/// Create chart relationship XML with additional style and color references (like WPS does)
pub fn create_chart_relationship_xml_with_styles(chart_number: usize, excel_filename: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" Target="../embeddings/{}"/>
  <Relationship Id="rId2" Type="http://schemas.microsoft.com/office/2011/relationships/chartStyle" Target="style{}.xml"/>
  <Relationship Id="rId3" Type="http://schemas.microsoft.com/office/2011/relationships/chartColorStyle" Target="colors{}.xml"/>
</Relationships>"#,
        excel_filename, chart_number, chart_number
    )
}

/// Create minimal chart style XML
pub fn create_chart_style_xml(_chart_number: usize) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cs:chartStyle xmlns:cs="http://schemas.microsoft.com/office/drawing/2012/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main">
  <cs:chartArea>
    <cs:lnRef idx="0" />
    <cs:fillRef idx="0" />
    <cs:effectRef idx="0" />
    <cs:fontRef idx="minor" />
  </cs:chartArea>
  <cs:plotArea>
    <cs:spPr>
      <a:noFill/>
      <a:ln>
        <a:noFill/>
      </a:ln>
      <a:effectLst/>
    </cs:spPr>
  </cs:plotArea>
  <cs:axis>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="15000"/>
            <a:lumOff val="85000"/>
          </a:schemeClr>
        </a:solidFill>
      </a:ln>
    </cs:spPr>
  </cs:axis>
  <cs:majorGridlines>
    <cs:spPr>
      <a:ln w="9525" cap="flat" cmpd="sng" algn="ctr">
        <a:solidFill>
          <a:schemeClr val="tx1">
            <a:lumMod val="5000"/>
            <a:lumOff val="95000"/>
          </a:schemeClr>
        </a:solidFill>
      </a:ln>
    </cs:spPr>
  </cs:majorGridlines>
</cs:chartStyle>"#
    )
}

/// Create minimal chart color style XML
pub fn create_chart_color_style_xml(_chart_number: usize) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cs:colorStyle xmlns:cs="http://schemas.microsoft.com/office/drawing/2012/chartStyle" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" meth="cycle" id="10">
  <cs:variation/>
  <cs:variation>
    <a:lumMod val="60000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="80000"/>
    <a:lumOff val="20000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="40000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="60000"/>
    <a:lumOff val="40000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="80000"/>
    <a:lumOff val="60000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="20000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="40000"/>
    <a:lumOff val="60000"/>
  </cs:variation>
  <cs:variation>
    <a:lumMod val="20000"/>
    <a:lumOff val="80000"/>
  </cs:variation>
</cs:colorStyle>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chart_relationship_xml() {
        let xml = create_chart_relationship_xml(1, "Workbook1.xlsx");
        assert!(xml.contains("rId1"));
        assert!(xml.contains("../embeddings/Workbook1.xlsx"));
        assert!(xml.contains("http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"));
    }

    #[test]
    fn test_create_chart_relationship_xml_with_styles() {
        let xml = create_chart_relationship_xml_with_styles(1, "Workbook1.xlsx");
        assert!(xml.contains("rId1"));
        assert!(xml.contains("rId2"));
        assert!(xml.contains("rId3"));
        assert!(xml.contains("../embeddings/Workbook1.xlsx"));
        assert!(xml.contains("style1.xml"));
        assert!(xml.contains("colors1.xml"));
    }

    #[test]
    fn test_create_chart_style_xml() {
        let xml = create_chart_style_xml(1);
        assert!(xml.contains("cs:chartStyle"));
        assert!(xml.contains("chartArea"));
        assert!(xml.contains("plotArea"));
    }

    #[test]
    fn test_create_chart_color_style_xml() {
        let xml = create_chart_color_style_xml(1);
        assert!(xml.contains("cs:colorStyle"));
        assert!(xml.contains("variation"));
    }
}