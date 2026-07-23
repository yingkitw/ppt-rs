//! Load theme / master / layout parts from an existing PPTX template.

use std::collections::HashMap;
use std::path::Path;

use crate::exc::{PptxError, Result};
use crate::opc::Package;

/// Theme + master + layout parts cloned from an existing `.pptx` file.
#[derive(Clone, Debug, Default)]
pub struct PptxTemplate {
    parts: HashMap<String, Vec<u8>>,
    layout_count: usize,
}

impl PptxTemplate {
    /// Load template parts from a `.pptx` on disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let pkg = Package::open(path)?;
        Self::from_package(&pkg)
    }

    /// Extract template parts from an opened package.
    pub fn from_package(pkg: &Package) -> Result<Self> {
        let mut parts = HashMap::new();
        for path in pkg.part_paths() {
            if (path.starts_with("ppt/theme/")
                || path.starts_with("ppt/slideMasters/")
                || path.starts_with("ppt/slideLayouts/")
                || path == "ppt/tableStyles.xml")
                && let Some(data) = pkg.get_part(path) {
                    parts.insert(path.to_string(), data.to_vec());
                }
        }

        if !parts.keys().any(|p| p.starts_with("ppt/slideMasters/")) {
            return Err(PptxError::InvalidValue(
                "template missing ppt/slideMasters/".into(),
            ));
        }

        let layout_count = parts
            .keys()
            .filter(|p| {
                p.starts_with("ppt/slideLayouts/slideLayout")
                    && p.ends_with(".xml")
                    && !p.contains("_rels")
            })
            .count()
            .max(1);

        Ok(PptxTemplate { parts, layout_count })
    }

    pub fn layout_count(&self) -> usize {
        self.layout_count
    }

    pub fn parts(&self) -> &HashMap<String, Vec<u8>> {
        &self.parts
    }

    pub fn has_layout(&self, n: usize) -> bool {
        self.parts
            .contains_key(&format!("ppt/slideLayouts/slideLayout{n}.xml"))
    }

    /// Resolve layout index for a slide, capped to layouts available in the template.
    pub fn resolve_layout_number(&self, requested: usize) -> usize {
        if self.has_layout(requested) {
            requested
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::create_pptx;

    #[test]
    fn load_template_from_generated_deck() {
        let bytes = create_pptx("Tpl", 1).unwrap();
        let dir = std::env::temp_dir().join("ppt_rs_template_test.pptx");
        std::fs::write(&dir, &bytes).unwrap();
        let tpl = PptxTemplate::load(&dir).unwrap();
        assert!(tpl.layout_count() >= 1);
        assert!(tpl.has_layout(1));
        std::fs::remove_file(dir).ok();
    }
}
