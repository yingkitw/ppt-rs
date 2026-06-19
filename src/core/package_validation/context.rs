//! Read-only view of a PPTX ZIP archive for validation rules.

use std::collections::HashMap;
use std::io::{Read, Seek};

use zip::ZipArchive;

use super::rels::{rels_source_dir, resolve_rel_target};
use super::report::{PackageValidationIssue, ValidationCategory};

#[derive(Debug, Default)]
pub struct Relationship {
    pub id: String,
    pub target: String,
    pub resolved: String,
    /// Raw `TargetMode` attribute value (e.g. `"External"`). Empty when absent,
    /// which means the relationship targets a part inside the package.
    pub target_mode: String,
}

/// Cached package contents used by validation rules.
pub struct PackageContext {
    pub names: std::collections::HashSet<String>,
    rels_by_path: HashMap<String, Vec<Relationship>>,
    part_cache: HashMap<String, String>,
}

impl PackageContext {
    pub fn from_archive<R: Read + Seek>(archive: &mut ZipArchive<R>) -> Self {
        let mut names = std::collections::HashSet::new();
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                if !file.is_dir() {
                    names.insert(file.name().to_string());
                }
            }
        }

        let mut ctx = Self {
            names,
            rels_by_path: HashMap::new(),
            part_cache: HashMap::new(),
        };

        let rels_paths: Vec<String> = ctx
            .names
            .iter()
            .filter(|n| n.ends_with(".rels"))
            .cloned()
            .collect();

        for rels_path in rels_paths {
            if let Ok(content) = ctx.read_part_from_archive(archive, &rels_path) {
                let base = rels_source_dir(&rels_path);
                let relationships = parse_relationships(&content, &base);
                ctx.rels_by_path.insert(rels_path, relationships);
            }
        }

        ctx
    }

    pub fn has_part(&self, path: &str) -> bool {
        self.names.contains(path)
    }

    pub fn relationships(&self, rels_path: &str) -> &[Relationship] {
        self.rels_by_path
            .get(rels_path)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn relationship_parts(&self) -> impl Iterator<Item = (&str, &[Relationship])> {
        self.rels_by_path
            .iter()
            .map(|(path, rels)| (path.as_str(), rels.as_slice()))
    }

    pub fn read_part<R: Read + Seek>(
        &mut self,
        archive: &mut ZipArchive<R>,
        path: &str,
    ) -> Option<String> {
        if let Some(cached) = self.part_cache.get(path) {
            return Some(cached.clone());
        }
        let content = self.read_part_from_archive(archive, path).ok()?;
        self.part_cache.insert(path.to_string(), content.clone());
        Some(content)
    }

    fn read_part_from_archive<R: Read + Seek>(
        &self,
        archive: &mut ZipArchive<R>,
        path: &str,
    ) -> Result<String, PackageValidationIssue> {
        let mut file = archive.by_name(path).map_err(|_| {
            PackageValidationIssue::error(
                ValidationCategory::MissingPart,
                format!("Cannot read part for validation: {path}"),
                Some(path),
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            PackageValidationIssue::error(
                ValidationCategory::Xml,
                format!("Failed to read {path}: {e}"),
                Some(path),
            )
        })?;
        Ok(content)
    }
}

fn parse_relationships(rels_xml: &str, base_dir: &str) -> Vec<Relationship> {
    // base_dir is the directory of the relationship source part
    let mut relationships = Vec::new();
    let mut search_from = 0;

    while let Some(id_start) = rels_xml[search_from..].find("Id=\"") {
        let abs_id = search_from + id_start + 4;
        let Some(id_end) = rels_xml[abs_id..].find('"') else {
            break;
        };
        let id = rels_xml[abs_id..abs_id + id_end].to_string();

        let target_needle = "Target=\"";
        let after_id = abs_id + id_end;
        let Some(target_start) = rels_xml[after_id..].find(target_needle) else {
            search_from = after_id;
            continue;
        };
        let abs_target = after_id + target_start + target_needle.len();
        let Some(target_end) = rels_xml[abs_target..].find('"') else {
            break;
        };
        let target = rels_xml[abs_target..abs_target + target_end].to_string();

        // Scope the `TargetMode` lookup to the remainder of this element so the
        // attribute of a later relationship is never mis-attributed.
        let element_tail_end = rels_xml[abs_target + target_end..]
            .find('>')
            .map(|p| abs_target + target_end + p)
            .unwrap_or(rels_xml.len());
        let element_tail = &rels_xml[abs_target + target_end..element_tail_end];
        let target_mode = if element_tail.contains("TargetMode=\"External\"") {
            "External".to_string()
        } else {
            String::new()
        };

        let resolved = resolve_rel_target(base_dir, &target);

        relationships.push(Relationship {
            id,
            target,
            resolved,
            target_mode,
        });

        search_from = element_tail_end;
    }

    relationships
}
