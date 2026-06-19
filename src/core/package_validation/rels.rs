//! Relationship path resolution for OPC packages.

/// Directory containing the source part for a `.rels` file.
pub fn rels_source_dir(rels_path: &str) -> String {
    if rels_path == "_rels/.rels" {
        return String::new();
    }
    let part_path = rels_path.replacen("/_rels/", "/", 1);
    let part_path = part_path.strip_suffix(".rels").unwrap_or(&part_path);
    part_path
        .rsplit_once('/')
        .map(|(dir, _)| dir.to_string())
        .unwrap_or_default()
}

/// Resolve a relationship `Target` relative to the source part directory.
pub fn resolve_rel_target(base_dir: &str, target: &str) -> String {
    if target.starts_with('/') {
        return target.trim_start_matches('/').to_string();
    }

    let mut parts: Vec<&str> = if base_dir.is_empty() {
        Vec::new()
    } else {
        base_dir.split('/').collect()
    };

    for segment in target.split('/') {
        match segment {
            ".." => {
                parts.pop();
            }
            "." | "" => {}
            name => parts.push(name),
        }
    }

    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_root_rels_resolve_from_root() {
        assert_eq!(rels_source_dir("_rels/.rels"), "");
        assert_eq!(
            resolve_rel_target("", "ppt/presentation.xml"),
            "ppt/presentation.xml"
        );
    }

    #[test]
    fn presentation_rels_resolve_under_ppt() {
        assert_eq!(
            rels_source_dir("ppt/_rels/presentation.xml.rels"),
            "ppt"
        );
        assert_eq!(
            resolve_rel_target("ppt", "slides/slide1.xml"),
            "ppt/slides/slide1.xml"
        );
    }

    #[test]
    fn slide_rels_resolve_media() {
        assert_eq!(
            rels_source_dir("ppt/slides/_rels/slide3.xml.rels"),
            "ppt/slides"
        );
        assert_eq!(
            resolve_rel_target("ppt/slides", "../media/image1.png"),
            "ppt/media/image1.png"
        );
    }
}
