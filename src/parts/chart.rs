//! Chart part

use crate::error::Result;
use crate::opc::constants::CONTENT_TYPE;
use crate::opc::part::{BasePart, Part};
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;

/// Chart part - contains chart data
pub struct ChartPart {
    base: BasePart,
}

impl ChartPart {
    /// Create a new chart part
    pub fn new(partname: PackURI) -> Result<Self> {
        let base = BasePart::new(CONTENT_TYPE::DML_CHART, partname)?;
        Ok(Self { base })
    }

    /// Create a new chart part with XML content
    pub fn with_xml(partname: PackURI, _xml_content: String) -> Result<Self> {
        let base = BasePart::new(CONTENT_TYPE::DML_CHART, partname)?;
        // TODO: Parse and store XML element
        Ok(Self { base })
    }
}

impl Part for ChartPart {
    fn content_type(&self) -> &str {
        self.base.content_type()
    }

    fn uri(&self) -> &PackURI {
        self.base.uri()
    }

    fn relationships(&self) -> &Relationships {
        self.base.relationships()
    }

    fn relationships_mut(&mut self) -> &mut Relationships {
        self.base.relationships_mut()
    }

    fn blob(&self) -> Result<Vec<u8>> {
        self.base.blob()
    }

    fn to_xml(&self) -> Result<String> {
        self.base.to_xml()
    }

    fn from_xml<R: std::io::Read>(reader: R) -> Result<Self> {
        let _xml_str = crate::oxml::parser::parse_xml(reader)?;
        // TODO: Parse XML and create ChartPart
        Self::new(PackURI::new("/ppt/charts/chart1.xml")?)
    }
}

