//! XML parsing and generation for shapes

use crate::error::Result;
use crate::shapes::{AutoShape, AutoShapeType, Connector, Picture, Shape};
use regex::Regex;

/// Parse shapes from slide XML spTree
pub fn parse_shapes_from_xml(xml: &str) -> Result<Vec<Box<dyn Shape>>> {
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
    
    // Find spTree element
    if !xml.contains("<p:spTree>") {
        return Ok(shapes);
    }
    
    // Extract all shape elements from spTree
    // Match: <p:sp>, <p:pic>, <p:cxnSp>, <p:grpSp>, <p:graphicFrame>
    // Use (?s) flag to make . match newlines
    
    // Parse <p:sp> elements (AutoShape)
    let sp_re = Regex::new(r"(?s)<p:sp[^>]*>.*?</p:sp>").ok();
    if let Some(re) = sp_re {
        for cap in re.find_iter(xml) {
            if let Ok(shape) = parse_autoshape_xml(cap.as_str()) {
                shapes.push(Box::new(shape));
            }
        }
    }
    
    // Parse <p:pic> elements (Picture)
    let pic_re = Regex::new(r"(?s)<p:pic[^>]*>.*?</p:pic>").ok();
    if let Some(re) = pic_re {
        for cap in re.find_iter(xml) {
            if let Ok(shape) = parse_picture_xml(cap.as_str()) {
                shapes.push(Box::new(shape));
            }
        }
    }
    
    // Parse <p:cxnSp> elements (Connector)
    let cxn_re = Regex::new(r"(?s)<p:cxnSp[^>]*>.*?</p:cxnSp>").ok();
    if let Some(re) = cxn_re {
        for cap in re.find_iter(xml) {
            if let Ok(shape) = parse_connector_xml(cap.as_str()) {
                shapes.push(Box::new(shape));
            }
        }
    }
    
    Ok(shapes)
}

/// Parse AutoShape from XML
fn parse_autoshape_xml(xml: &str) -> Result<AutoShape> {
    // Extract id from nvSpPr/cNvPr
    let id_re = Regex::new(r#"<p:cNvPr\s+id="(\d+)""#).ok();
    let id = id_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(1);
    
    // Extract name
    let name_re = Regex::new(r#"name="([^"]+)""#).ok();
    let name = name_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| format!("Shape {}", id));
    
    // Extract position and size from xfrm
    let (left, top, width, height) = parse_shape_transform(xml);
    
    // Create AutoShape (default to Rectangle)
    let mut shape = AutoShape::new(id, name, AutoShapeType::Rectangle);
    shape.set_left(left);
    shape.set_top(top);
    shape.set_width(width);
    shape.set_height(height);
    
    Ok(shape)
}

/// Parse Picture from XML
fn parse_picture_xml(xml: &str) -> Result<Picture> {
    // Extract id from nvPicPr/cNvPr
    let id_re = Regex::new(r#"<p:cNvPr\s+id="(\d+)""#).ok();
    let id = id_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(1);
    
    // Extract name
    let name_re = Regex::new(r#"name="([^"]+)""#).ok();
    let name = name_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| format!("Picture {}", id));
    
    // Extract position and size
    let (left, top, width, height) = parse_shape_transform(xml);
    
    let mut pic = Picture::new(id, name);
    pic.set_left(left);
    pic.set_top(top);
    pic.set_width(width);
    pic.set_height(height);
    
    Ok(pic)
}

/// Parse Connector from XML
fn parse_connector_xml(xml: &str) -> Result<Connector> {
    // Extract id from nvCxnSpPr/cNvPr
    let id_re = Regex::new(r#"<p:cNvPr\s+id="(\d+)""#).ok();
    let id = id_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok())
        .unwrap_or(1);
    
    // Extract name
    let name_re = Regex::new(r#"name="([^"]+)""#).ok();
    let name = name_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| format!("Connector {}", id));
    
    // Extract position and size
    let (left, top, width, height) = parse_shape_transform(xml);
    
    // Extract start/end shape IDs if present
    let start_id_re = Regex::new(r#"stCxn\s+id="(\d+)""#).ok();
    let end_id_re = Regex::new(r#"endCxn\s+id="(\d+)""#).ok();
    
    let start_id = start_id_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());
    
    let end_id = end_id_re
        .and_then(|re| re.captures(xml))
        .and_then(|cap| cap.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());
    
    let mut conn = if let (Some(sid), Some(eid)) = (start_id, end_id) {
        Connector::between(id, name, sid, eid)
    } else {
        Connector::new(id, name)
    };
    
    conn.set_left(left);
    conn.set_top(top);
    conn.set_width(width);
    conn.set_height(height);
    
    Ok(conn)
}

/// Parse shape transform (position and size) from XML
fn parse_shape_transform(xml: &str) -> (i64, i64, u32, u32) {
    // Look for <a:xfrm> element with <a:off> and <a:ext>
    let off_re = Regex::new(r#"<a:off\s+x="(\d+)"\s+y="(\d+)""#).ok();
    let ext_re = Regex::new(r#"<a:ext\s+cx="(\d+)"\s+cy="(\d+)""#).ok();
    
    let (left, top) = off_re
        .and_then(|re| re.captures(xml))
        .map(|cap| {
            let x = cap.get(1).and_then(|m| m.as_str().parse::<i64>().ok()).unwrap_or(0);
            let y = cap.get(2).and_then(|m| m.as_str().parse::<i64>().ok()).unwrap_or(0);
            (x, y)
        })
        .unwrap_or((0, 0));
    
    let (width, height) = ext_re
        .and_then(|re| re.captures(xml))
        .map(|cap| {
            let cx = cap.get(1).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(914400);
            let cy = cap.get(2).and_then(|m| m.as_str().parse::<u32>().ok()).unwrap_or(914400);
            (cx, cy)
        })
        .unwrap_or((914400, 914400));
    
    (left, top, width, height)
}

/// Generate XML for a shape
pub fn shape_to_xml(shape: &dyn Shape, next_shape_id: u32) -> String {
    // Use the shape's ID or next_shape_id if shape doesn't have valid ID
    let shape_id = if shape.id() > 0 { shape.id() } else { next_shape_id };
    let name = shape.name();
    
    // Generate basic shape XML based on shape type
    // This is a simplified version - full implementation would handle all shape types
    format!(
        r#"<p:sp>
      <p:nvSpPr>
        <p:cNvPr id="{}" name="{}"/>
        <p:cNvSpPr/>
        <p:nvPr/>
      </p:nvSpPr>
      <p:spPr>
        <a:xfrm>
          <a:off x="{}" y="{}"/>
          <a:ext cx="{}" cy="{}"/>
        </a:xfrm>
        <a:prstGeom prst="rect">
          <a:avLst/>
        </a:prstGeom>
      </p:spPr>
    </p:sp>"#,
        shape_id,
        name,
        shape.left(),
        shape.top(),
        shape.width(),
        shape.height()
    )
}

/// Find the next available shape ID from XML
pub fn next_shape_id(xml: &str) -> u32 {
    let id_re = Regex::new(r#"id="(\d+)""#).ok();
    let mut max_id = 0u32;
    
    if let Some(re) = id_re {
        for cap in re.captures_iter(xml) {
            if let Some(id_str) = cap.get(1) {
                if let Ok(id) = id_str.as_str().parse::<u32>() {
                    if id > max_id {
                        max_id = id;
                    }
                }
            }
        }
    }
    
    max_id + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{AutoShape, AutoShapeType, Picture, Connector};

    #[test]
    fn test_parse_shapes_from_xml_empty() {
        let xml = "<p:sld></p:sld>";
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_parse_shapes_from_xml_no_sptree() {
        let xml = "<p:sld><p:cSld></p:cSld></p:sld>";
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 0);
    }

    #[test]
    fn test_parse_shapes_from_xml_autoshape() {
        let xml = r#"<p:spTree>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="2" name="Rectangle 1"/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="1000000" y="2000000"/>
                        <a:ext cx="3000000" cy="4000000"/>
                    </a:xfrm>
                </p:spPr>
            </p:sp>
        </p:spTree>"#;
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id(), 2);
        assert_eq!(shapes[0].left(), 1000000);
        assert_eq!(shapes[0].top(), 2000000);
        assert_eq!(shapes[0].width(), 3000000);
        assert_eq!(shapes[0].height(), 4000000);
    }

    #[test]
    fn test_parse_shapes_from_xml_picture() {
        let xml = r#"<p:spTree>
            <p:pic>
                <p:nvPicPr>
                    <p:cNvPr id="3" name="Picture 1"/>
                </p:nvPicPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="500000" y="600000"/>
                        <a:ext cx="2000000" cy="1500000"/>
                    </a:xfrm>
                </p:spPr>
            </p:pic>
        </p:spTree>"#;
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id(), 3);
        assert_eq!(shapes[0].name(), "Picture 1");
    }

    #[test]
    fn test_parse_shapes_from_xml_connector() {
        let xml = r#"<p:spTree>
            <p:cxnSp>
                <p:nvCxnSpPr>
                    <p:cNvPr id="4" name="Connector 1"/>
                </p:nvCxnSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="0" y="0"/>
                        <a:ext cx="1000000" cy="1000000"/>
                    </a:xfrm>
                </p:spPr>
            </p:cxnSp>
        </p:spTree>"#;
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 1);
        assert_eq!(shapes[0].id(), 4);
    }

    #[test]
    fn test_parse_shapes_from_xml_multiple() {
        let xml = r#"<p:spTree>
            <p:sp>
                <p:nvSpPr>
                    <p:cNvPr id="1" name="Shape 1"/>
                </p:nvSpPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="0" y="0"/>
                        <a:ext cx="1000000" cy="1000000"/>
                    </a:xfrm>
                </p:spPr>
            </p:sp>
            <p:pic>
                <p:nvPicPr>
                    <p:cNvPr id="2" name="Picture 1"/>
                </p:nvPicPr>
                <p:spPr>
                    <a:xfrm>
                        <a:off x="0" y="0"/>
                        <a:ext cx="1000000" cy="1000000"/>
                    </a:xfrm>
                </p:spPr>
            </p:pic>
        </p:spTree>"#;
        let shapes = parse_shapes_from_xml(xml).unwrap();
        assert_eq!(shapes.len(), 2);
    }

    #[test]
    fn test_shape_to_xml() {
        let mut shape = AutoShape::new(5, "Test Shape".to_string(), AutoShapeType::Rectangle);
        shape.set_left(100000);
        shape.set_top(200000);
        shape.set_width(300000);
        shape.set_height(400000);
        
        let xml = shape_to_xml(&shape, 10);
        assert!(xml.contains("id=\"5\""));
        assert!(xml.contains("name=\"Test Shape\""));
        assert!(xml.contains("x=\"100000\""));
        assert!(xml.contains("y=\"200000\""));
        assert!(xml.contains("cx=\"300000\""));
        assert!(xml.contains("cy=\"400000\""));
    }

    #[test]
    fn test_shape_to_xml_uses_next_id() {
        use crate::shapes::base::BaseShape;
        let shape = BaseShape::new(0, "Shape".to_string());
        let xml = shape_to_xml(&shape, 99);
        assert!(xml.contains("id=\"99\""));
    }

    #[test]
    fn test_next_shape_id() {
        let xml = r#"<p:cNvPr id="1" name="Shape 1"/>
                     <p:cNvPr id="5" name="Shape 2"/>
                     <p:cNvPr id="3" name="Shape 3"/>"#;
        assert_eq!(next_shape_id(xml), 6);
    }

    #[test]
    fn test_next_shape_id_no_ids() {
        let xml = "<p:sld></p:sld>";
        assert_eq!(next_shape_id(xml), 1);
    }

    #[test]
    fn test_parse_shape_transform() {
        let xml = r#"<a:xfrm>
            <a:off x="1000000" y="2000000"/>
            <a:ext cx="3000000" cy="4000000"/>
        </a:xfrm>"#;
        let (left, top, width, height) = parse_shape_transform(xml);
        assert_eq!(left, 1000000);
        assert_eq!(top, 2000000);
        assert_eq!(width, 3000000);
        assert_eq!(height, 4000000);
    }

    #[test]
    fn test_parse_shape_transform_defaults() {
        let xml = "<p:sp></p:sp>";
        let (left, top, width, height) = parse_shape_transform(xml);
        assert_eq!(left, 0);
        assert_eq!(top, 0);
        assert_eq!(width, 914400);
        assert_eq!(height, 914400);
    }
}

