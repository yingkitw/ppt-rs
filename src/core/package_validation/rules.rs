//! Composable validation rules for generated PPTX packages.

use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek};

use zip::ZipArchive;

use crate::core::validation::validate_well_formed_xml;
use crate::generator::package_xml::{first_slide_rel_id, slide_id_value};

use super::context::PackageContext;
use super::report::{
    PackageValidationIssue, PackageValidationReport, ValidationCategory,
};

/// Parts PowerPoint expects in every generated deck.
pub const REQUIRED_PACKAGE_PARTS: &[&str] = &[
    "[Content_Types].xml",
    "_rels/.rels",
    "ppt/presentation.xml",
    "ppt/_rels/presentation.xml.rels",
    "ppt/presProps.xml",
    "ppt/viewProps.xml",
    "ppt/tableStyles.xml",
    "ppt/theme/theme1.xml",
    "ppt/slideMasters/slideMaster1.xml",
    "ppt/slideLayouts/slideLayout1.xml",
    "docProps/core.xml",
    "docProps/app.xml",
];

/// Validate a PPTX archive using all structural rules.
pub fn validate_package<R: Read + Seek>(archive: &mut ZipArchive<R>) -> PackageValidationReport {
    let mut ctx = PackageContext::from_archive(archive);
    let mut report = PackageValidationReport::default();

    check_required_parts(&ctx, &mut report);
    check_xml_parts(&mut ctx, archive, &mut report);
    check_relationship_targets(&ctx, &mut report);
    check_content_types(&mut ctx, archive, &mut report);
    check_presentation_structure(&mut ctx, archive, &mut report);
    check_slide_master(&mut ctx, archive, &mut report);
    check_theme(&mut ctx, archive, &mut report);
    check_chart_packages(&mut ctx, archive, &mut report);
    check_slide_relationships(&mut ctx, archive, &mut report);
    check_notes_slides(&ctx, &mut report);
    check_handout_package(&mut ctx, archive, &mut report);
    check_notes_master(&mut ctx, archive, &mut report);

    report
}

/// Validate PPTX bytes (opens a ZIP archive internally).
pub fn validate_package_bytes(bytes: &[u8]) -> PackageValidationReport {
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = match ZipArchive::new(cursor) {
        Ok(a) => a,
        Err(e) => {
            let mut report = PackageValidationReport::default();
            report.push(PackageValidationIssue::error(
                ValidationCategory::Xml,
                format!("Invalid ZIP archive: {e}"),
                None,
            ));
            return report;
        }
    };
    validate_package(&mut archive)
}

fn check_required_parts(ctx: &PackageContext, report: &mut PackageValidationReport) {
    for part in REQUIRED_PACKAGE_PARTS {
        if !ctx.has_part(part) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::MissingPart,
                format!("Missing required part: {part}"),
                Some(part),
            ));
        }
    }
}

fn check_xml_parts<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let xml_parts: Vec<String> = ctx
        .names
        .iter()
        .filter(|n| n.ends_with(".xml") || n.ends_with(".rels"))
        .cloned()
        .collect();

    for path in xml_parts {
        if let Some(content) = ctx.read_part(archive, &path) {
            if let Err(e) = validate_well_formed_xml(&content) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Xml,
                    e.to_string(),
                    Some(&path),
                ));
            }
        }
    }
}

fn check_relationship_targets(ctx: &PackageContext, report: &mut PackageValidationReport) {
    for (rels_path, relationships) in ctx.relationship_parts() {
        for rel in relationships {
            // External relationships (e.g. hyperlinks to URLs) point outside the
            // package and therefore have no resolvable part to validate.
            if rel.target_mode == "External" {
                continue;
            }
            if !ctx.has_part(&rel.resolved) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Relationship,
                    format!(
                        "{} Id=\"{}\" Target=\"{}\" resolves to missing part \"{}\"",
                        rels_path, rel.id, rel.target, rel.resolved
                    ),
                    Some(rels_path),
                ));
            }
        }
    }
}

fn check_content_types<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let Some(ct) = ctx.read_part(archive, "[Content_Types].xml") else {
        return;
    };

    for path in &ctx.names {
        if path.ends_with(".rels") || path == "[Content_Types].xml" {
            continue;
        }
        let override_path = format!("/{path}");
        if ct.contains(&override_path) {
            continue;
        }
        let ext = path.rsplit('.').next().unwrap_or("");
        let default_needle = format!("Extension=\"{ext}\"");
        if ext.is_empty() || !ct.contains(&default_needle) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::ContentType,
                format!("Part {path} has no Override or Default in [Content_Types].xml"),
                Some(path),
            ));
        }
    }
}

fn check_presentation_structure<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let Some(pres) = ctx.read_part(archive, "ppt/presentation.xml") else {
        return;
    };
    let pres_rels = ctx.relationships("ppt/_rels/presentation.xml.rels");
    let rel_by_id: HashMap<&str, &str> = pres_rels
        .iter()
        .map(|r| (r.id.as_str(), r.resolved.as_str()))
        .collect();

    let slide_parts: usize = ctx
        .names
        .iter()
        .filter(|n| is_slide_part(n))
        .count();
    let sld_id_count = pres.matches("<p:sldId ").count();

    if sld_id_count > 0 {
        let first_id = format!("id=\"{}\"", slide_id_value(1));
        if !pres.contains(&first_id) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Presentation,
                format!(
                    "First slide id should be {} (PowerPoint convention)",
                    slide_id_value(1)
                ),
                Some("ppt/presentation.xml"),
            ));
        }

        let has_notes = pres.contains("<p:notesMasterIdLst>");
        let has_handout = pres.contains("<p:handoutMasterIdLst>");
        let expected_first_rid = first_slide_rel_id(has_notes, has_handout);
        let first_rid = format!(r#"r:id="rId{expected_first_rid}""#);
        if pres.contains("<p:sldIdLst>") && !pres.contains(&first_rid) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Presentation,
                format!(
                    "First slide r:id should be rId{expected_first_rid} for this master's optional parts"
                ),
                Some("ppt/presentation.xml"),
            ));
        }
    }

    validate_presentation_rel_order(pres_rels, report);

    for sld_rid in extract_attr_values(&pres, "p:sldId", "r:id") {
        let target = rel_by_id.get(sld_rid.as_str());
        match target {
            Some(t) if t.contains("/slides/slide") => {}
            _ => report.push(PackageValidationIssue::error(
                ValidationCategory::Presentation,
                format!("Slide r:id=\"{sld_rid}\" does not resolve to a slide part"),
                Some("ppt/presentation.xml"),
            )),
        }
    }

    let slide_parts: usize = ctx
        .names
        .iter()
        .filter(|n| is_slide_part(n))
        .count();
    if slide_parts != sld_id_count {
        report.push(PackageValidationIssue::error(
            ValidationCategory::Presentation,
            format!(
                "Slide count mismatch: {slide_parts} slide parts vs {sld_id_count} p:sldId entries"
            ),
            Some("ppt/presentation.xml"),
        ));
    }
}

fn validate_presentation_rel_order(
    pres_rels: &[super::context::Relationship],
    report: &mut PackageValidationReport,
) {
    let rels_xml = pres_rels
        .iter()
        .map(|r| format!("Id=\"{}\" Target=\"{}\"", r.id, r.target))
        .collect::<Vec<_>>()
        .join(" ");

    if !rels_xml.contains("Id=\"rId1\"") || !rels_xml.contains("slideMaster") {
        report.push(PackageValidationIssue::error(
            ValidationCategory::Relationship,
            "presentation.xml.rels missing rId1 (slideMaster)",
            Some("ppt/_rels/presentation.xml.rels"),
        ));
    }

    let slide_pos = rels_xml.find("slides/slide");
    let pres_props_pos = rels_xml.find("presProps");
    if let (Some(s), Some(p)) = (slide_pos, pres_props_pos) {
        if s > p {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Relationship,
                "Slide relationships should appear before presProps in presentation.xml.rels",
                Some("ppt/_rels/presentation.xml.rels"),
            ));
        }
    }

    let table_pos = rels_xml.find("tableStyles");
    let theme_pos = rels_xml.find("theme/theme1");
    if let (Some(t), Some(tb)) = (theme_pos, table_pos) {
        if t > tb {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Relationship,
                "theme should appear before tableStyles in presentation.xml.rels",
                Some("ppt/_rels/presentation.xml.rels"),
            ));
        }
    }
}

fn check_slide_master<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let Some(master) = ctx.read_part(archive, "ppt/slideMasters/slideMaster1.xml") else {
        return;
    };

    if !master.contains("<p:txStyles>") {
        report.push(PackageValidationIssue::error(
            ValidationCategory::SlideMaster,
            "Slide master missing p:txStyles",
            Some("ppt/slideMasters/slideMaster1.xml"),
        ));
    }

    let Some(pres) = ctx.read_part(archive, "ppt/presentation.xml") else {
        return;
    };
    check_master_id_uniqueness(&pres, &master, report);

    let master_rels = ctx.relationships("ppt/slideMasters/_rels/slideMaster1.xml.rels");
    let rel_by_id: HashMap<&str, &str> = master_rels
        .iter()
        .map(|r| (r.id.as_str(), r.resolved.as_str()))
        .collect();

    for layout_rid in extract_attr_values(&master, "p:sldLayoutId", "r:id") {
        let target = rel_by_id.get(layout_rid.as_str());
        match target {
            Some(t) if t.contains("slideLayout") => {}
            _ => report.push(PackageValidationIssue::error(
                ValidationCategory::SlideMaster,
                format!("Layout r:id=\"{layout_rid}\" does not resolve to a slideLayout part"),
                Some("ppt/slideMasters/slideMaster1.xml"),
            )),
        }
    }

    // PowerPoint omits id on notes/handout master entries; flag collisions if present.
}

fn check_master_id_uniqueness(
    pres: &str,
    master: &str,
    report: &mut PackageValidationReport,
) {
    let layout_ids: HashSet<String> = extract_attr_values(master, "p:sldLayoutId", "id")
        .into_iter()
        .collect();

    for (tag, label) in [
        ("notesMasterId", "notes master"),
        ("handoutMasterId", "handout master"),
    ] {
        for id in extract_attr_values(pres, tag, "id") {
            if layout_ids.contains(&id) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::SlideMaster,
                    format!("{label} id {id} collides with a slide layout id on the slide master"),
                    Some("ppt/presentation.xml"),
                ));
            }
        }
    }
}

fn check_theme<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let mut buf = Vec::new();
    if let Ok(mut file) = archive.by_name("ppt/theme/theme1.xml") {
        if file.read_to_end(&mut buf).is_ok() && buf.len() < 7000 {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Theme,
                format!(
                    "Theme part too small ({} bytes); PowerPoint expects full Office theme",
                    buf.len()
                ),
                Some("ppt/theme/theme1.xml"),
            ));
        }
    }
    let _ = ctx;
}

fn check_chart_packages<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let chart_parts: Vec<String> = ctx
        .names
        .iter()
        .filter(|n| n.starts_with("ppt/charts/chart") && n.ends_with(".xml"))
        .cloned()
        .collect();

    for chart_path in chart_parts {
        let rels_path = chart_path.replace("ppt/charts/", "ppt/charts/_rels/") + ".rels";
        if !ctx.has_part(&rels_path) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Chart,
                format!("Chart missing rels: {rels_path}"),
                Some(&chart_path),
            ));
            continue;
        }

        let rels = ctx.relationships(&rels_path);
        let has_embedding = rels.iter().any(|r| r.target.contains("embeddings/"));
        if !has_embedding {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Chart,
                format!("{rels_path} missing package relationship to embedding"),
                Some(&rels_path),
            ));
        }

        if let Some(chart_xml) = ctx.read_part(archive, &chart_path) {
            if !chart_xml.contains("<c:externalData") {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Chart,
                    format!("{chart_path} missing c:externalData"),
                    Some(&chart_path),
                ));
            }
        }

        let idx = chart_path
            .trim_start_matches("ppt/charts/chart")
            .trim_end_matches(".xml");
        let embedding = format!("ppt/embeddings/Microsoft_Excel_Sheet{idx}.xlsx");
        if !ctx.has_part(&embedding) {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Chart,
                format!("Chart missing embedding: {embedding}"),
                Some(&chart_path),
            ));
        }
    }
}

fn check_slide_relationships<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    let slide_paths: Vec<String> = ctx
        .names
        .iter()
        .filter(|n| is_slide_part(n))
        .cloned()
        .collect();

    for slide_path in slide_paths {
        let rels_path = slide_path.replace("ppt/slides/", "ppt/slides/_rels/") + ".rels";
        let rels = ctx.relationships(&rels_path);
        let rel_map: HashMap<String, (String, String)> = rels
            .iter()
            .map(|r| (r.id.clone(), (r.resolved.clone(), r.target_mode.clone())))
            .collect();

        let Some(slide_xml) = ctx.read_part(archive, &slide_path) else {
            continue;
        };

        for rid in extract_rel_reference_ids(&slide_xml) {
            let Some((target, target_mode)) = rel_map.get(&rid) else {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Slide,
                    format!("{slide_path} references {rid} but {rels_path} has no such relationship"),
                    Some(&slide_path),
                ));
                continue;
            };
            // External relationships (e.g. URL hyperlinks) have no in-package target.
            if target_mode == "External" {
                continue;
            }
            if !ctx.has_part(target) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Slide,
                    format!("{slide_path} {rid} target \"{target}\" is missing from the package"),
                    Some(&slide_path),
                ));
            }
        }

        let mut shape_ids: HashSet<String> = HashSet::new();
        for id in extract_attr_values(&slide_xml, "p:cNvPr", "id") {
            if !shape_ids.insert(id.clone()) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Slide,
                    format!("{slide_path} has duplicate shape id \"{id}\""),
                    Some(&slide_path),
                ));
            }
        }
    }
}

fn check_handout_package<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    if !ctx.has_part("ppt/handoutMasters/handoutMaster1.xml") {
        return;
    }

    if let Some(pres_props) = ctx.read_part(archive, "ppt/presProps.xml") {
        if pres_props.contains("<p:prnPr") {
            report.push(PackageValidationIssue::error(
                ValidationCategory::Presentation,
                "presProps.xml must not contain p:prnPr when a handout master is packaged",
                Some("ppt/presProps.xml"),
            ));
        }
    }

    let handout_rels = ctx.relationships("ppt/handoutMasters/_rels/handoutMaster1.xml.rels");
    let has_theme3 = handout_rels
        .iter()
        .any(|r| r.resolved.contains("theme/theme3.xml"));
    if !has_theme3 {
        report.push(PackageValidationIssue::error(
            ValidationCategory::Relationship,
            "handout master should reference theme/theme3.xml",
            Some("ppt/handoutMasters/_rels/handoutMaster1.xml.rels"),
        ));
    }

    if let Some(handout) = ctx.read_part(archive, "ppt/handoutMasters/handoutMaster1.xml") {
        if !handout.contains("<p:bg>") {
            report.push(PackageValidationIssue::error(
                ValidationCategory::SlideMaster,
                "handout master missing slide background",
                Some("ppt/handoutMasters/handoutMaster1.xml"),
            ));
        }
    }
}

fn check_notes_master<R: Read + Seek>(
    ctx: &mut PackageContext,
    archive: &mut ZipArchive<R>,
    report: &mut PackageValidationReport,
) {
    if !ctx.has_part("ppt/notesMasters/notesMaster1.xml") {
        return;
    }

    let notes_rels = ctx.relationships("ppt/notesMasters/_rels/notesMaster1.xml.rels");
    let has_theme2 = notes_rels
        .iter()
        .any(|r| r.resolved.contains("theme/theme2.xml"));
    if !has_theme2 {
        report.push(PackageValidationIssue::error(
            ValidationCategory::Relationship,
            "notes master should reference theme/theme2.xml",
            Some("ppt/notesMasters/_rels/notesMaster1.xml.rels"),
        ));
    }

    if let Some(notes) = ctx.read_part(archive, "ppt/notesMasters/notesMaster1.xml") {
        if !notes.contains("<p:bg>") {
            report.push(PackageValidationIssue::error(
                ValidationCategory::SlideMaster,
                "notes master missing slide background",
                Some("ppt/notesMasters/notesMaster1.xml"),
            ));
        }
    }
}

fn check_notes_slides(ctx: &PackageContext, report: &mut PackageValidationReport) {
    for (rels_path, rels) in ctx.relationship_parts() {
        if !rels_path.starts_with("ppt/slides/_rels/") {
            continue;
        }
        for rel in rels {
            if rel.resolved.contains("notesSlides/notesSlide") && !ctx.has_part(&rel.resolved) {
                report.push(PackageValidationIssue::error(
                    ValidationCategory::Slide,
                    format!(
                        "{rels_path} references notes slide \"{}\" which is missing",
                        rel.resolved
                    ),
                    Some(rels_path),
                ));
            }
        }
    }
}

fn is_slide_part(path: &str) -> bool {
    path.starts_with("ppt/slides/slide")
        && path.ends_with(".xml")
        && !path.contains("/_rels/")
}

/// Extract `attr="value"` from elements like `<p:sldId id="256" r:id="rId6"/>`.
fn extract_attr_values(xml: &str, element: &str, attr: &str) -> Vec<String> {
    let mut values = Vec::new();
    let needle = format!("<{element}");
    let attr_needle = format!("{attr}=\"");
    let mut search_from = 0;

    while let Some(start) = xml[search_from..].find(&needle) {
        let abs = search_from + start;
        let end = xml[abs..]
            .find("/>")
            .or_else(|| xml[abs..].find("</"))
            .map(|i| abs + i)
            .unwrap_or(xml.len());
        let slice = &xml[abs..end];
        if let Some(attr_start) = slice.find(&attr_needle) {
            let val_start = attr_start + attr_needle.len();
            if let Some(val_end) = slice[val_start..].find('"') {
                values.push(slice[val_start..val_start + val_end].to_string());
            }
        }
        search_from = end;
    }

    values
}

/// Collect `r:embed` and chart-style `r:id` references from slide XML.
fn extract_rel_reference_ids(slide_xml: &str) -> Vec<String> {
    let mut ids = Vec::new();
    for prefix in ["r:embed=\"", "r:id=\""] {
        let mut search_from = 0;
        while let Some(start) = slide_xml[search_from..].find(prefix) {
            let abs = search_from + start + prefix.len();
            if let Some(end) = slide_xml[abs..].find('"') {
                let id = &slide_xml[abs..abs + end];
                if id.starts_with("rId") {
                    ids.push(id.to_string());
                }
            }
            search_from = abs;
        }
    }
    ids.sort();
    ids.dedup();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::create_pptx;

    #[test]
    fn minimal_generated_deck_passes_all_rules() {
        let bytes = create_pptx("Rules", 2).unwrap();
        let report = validate_package_bytes(&bytes);
        assert!(
            report.is_valid(),
            "expected valid package, got: {:?}",
            report.issues
        );
    }

    #[test]
    fn extract_rel_reference_ids_dedupes() {
        let xml = r#"<p:sld><p:pic><a:blip r:embed="rId2"/></p:pic><c:chart r:id="rId3"/></p:sld>"#;
        assert_eq!(
            extract_rel_reference_ids(xml),
            vec!["rId2".to_string(), "rId3".to_string()]
        );
    }
}
