//! Paragraph - a block of text with alignment and spacing

use super::run::Run;
use super::TextAlign;
use crate::core::ToXml;

/// A paragraph containing one or more runs
#[derive(Clone, Debug)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub align: TextAlign,
    pub level: u32,
    pub bullet: bool,
    pub spacing_before: Option<u32>,
    pub spacing_after: Option<u32>,
    pub line_spacing: Option<u32>,
}

impl Paragraph {
    /// Create a new empty paragraph
    pub fn new() -> Self {
        Paragraph {
            runs: Vec::new(),
            align: TextAlign::Left,
            level: 0,
            bullet: false,
            spacing_before: None,
            spacing_after: None,
            line_spacing: None,
        }
    }

    /// Create a paragraph with text
    pub fn with_text(text: &str) -> Self {
        let mut p = Self::new();
        p.runs.push(Run::new(text));
        p
    }

    /// Add a run
    pub fn add_run(mut self, run: Run) -> Self {
        self.runs.push(run);
        self
    }

    /// Add plain text
    pub fn add_text(mut self, text: &str) -> Self {
        self.runs.push(Run::new(text));
        self
    }

    /// Set alignment
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Set as bullet point
    pub fn bullet(mut self) -> Self {
        self.bullet = true;
        self
    }

    /// Set indent level (0-8)
    pub fn level(mut self, level: u32) -> Self {
        self.level = level.min(8);
        self
    }

    /// Set spacing before (in points)
    pub fn spacing_before(mut self, points: u32) -> Self {
        self.spacing_before = Some(points * 100);
        self
    }

    /// Set spacing after (in points)
    pub fn spacing_after(mut self, points: u32) -> Self {
        self.spacing_after = Some(points * 100);
        self
    }

    /// Generate XML for this paragraph
    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<a:p>");
        
        // Paragraph properties
        let mut ppr = format!(r#"<a:pPr algn="{}" lvl="{}""#, self.align.to_xml(), self.level);
        
        if self.spacing_before.is_some() || self.spacing_after.is_some() || self.line_spacing.is_some() {
            ppr.push('>');
            if let Some(before) = self.spacing_before {
                ppr.push_str(&format!(r#"<a:spcBef><a:spcPts val="{}"/></a:spcBef>"#, before));
            }
            if let Some(after) = self.spacing_after {
                ppr.push_str(&format!(r#"<a:spcAft><a:spcPts val="{}"/></a:spcAft>"#, after));
            }
            if self.bullet {
                ppr.push_str("<a:buChar char=\"•\"/>");
            }
            ppr.push_str("</a:pPr>");
        } else if self.bullet {
            ppr.push_str("><a:buChar char=\"•\"/></a:pPr>");
        } else {
            ppr.push_str("/>");
        }
        
        xml.push_str(&ppr);
        
        // Runs
        for run in &self.runs {
            xml.push_str(&run.to_xml());
        }
        
        xml.push_str("</a:p>");
        xml
    }
}

impl ToXml for Paragraph {
    fn to_xml(&self) -> String {
        Paragraph::to_xml(self)
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paragraph_to_xml() {
        let para = Paragraph::new()
            .add_run(Run::new("Bold text").bold())
            .add_run(Run::new(" normal text"))
            .align(TextAlign::Center);
        
        let xml = para.to_xml();
        
        assert!(xml.contains("<a:p>"));
        assert!(xml.contains("algn=\"ctr\""));
        assert!(xml.contains("Bold text"));
        assert!(xml.contains("normal text"));
    }

    #[test]
    fn test_paragraph_with_bullet() {
        let para = Paragraph::with_text("Bullet item").bullet();
        let xml = para.to_xml();
        
        assert!(xml.contains("buChar"));
    }
}
