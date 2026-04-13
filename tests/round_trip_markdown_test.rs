use ppt_rs::api::Presentation;
use ppt_rs::cli::parse_markdown;
use ppt_rs::generator::create_pptx_with_content;

fn fingerprint(pres: &Presentation) -> String {
    let mut out = String::new();

    for (idx, slide) in pres.slides().iter().enumerate() {
        let slide_no = idx + 1;
        out.push_str(&format!("slide: {slide_no}\n"));
        out.push_str(&format!("title: {}\n", slide.title));
        out.push_str("bullets:\n");
        for b in &slide.content {
            out.push_str(&format!("- {}\n", b));
        }
        out.push('\n');

        if let Some(table) = &slide.table {
            out.push_str("table:\n");
            for row in &table.rows {
                let cells: Vec<String> = row
                    .cells
                    .iter()
                    .map(|c| c.text.trim().to_string())
                    .collect();
                out.push_str(&format!("| {} |\n", cells.join(" | ")));
            }
            out.push('\n');
        }
    }

    out
}

fn round_trip_markdown_fixture(fixture_name: &str) -> String {
    let md_path = format!("tests/fixtures/markdown/{fixture_name}.md");
    let md = std::fs::read_to_string(&md_path)
        .unwrap_or_else(|e| panic!("failed to read {md_path}: {e}"));

    let slides = parse_markdown(&md)
        .unwrap_or_else(|e| panic!("parse_markdown failed for {md_path}: {e}"));

    let pptx_bytes = create_pptx_with_content("Round Trip Test", slides)
        .unwrap_or_else(|e| panic!("create_pptx_with_content failed: {e}"));

    let tmp_path = std::env::temp_dir()
        .join(format!("ppt_rs_round_trip_{}.pptx", uuid::Uuid::new_v4()));
    std::fs::write(&tmp_path, &pptx_bytes)
        .unwrap_or_else(|e| panic!("failed to write temp pptx {}: {e}", tmp_path.display()));

    let imported = Presentation::from_path(&tmp_path)
        .unwrap_or_else(|e| panic!("import failed for {}: {e}", tmp_path.display()));

    // Best-effort cleanup
    let _ = std::fs::remove_file(&tmp_path);

    fingerprint(&imported)
}

fn assert_matches_golden(fixture_name: &str) {
    let actual = round_trip_markdown_fixture(fixture_name);

    let golden_path = format!("tests/goldens/markdown/{fixture_name}.txt");
    let expected = std::fs::read_to_string(&golden_path)
        .unwrap_or_else(|e| panic!("failed to read golden {golden_path}: {e}"));

    assert_eq!(actual, expected, "round-trip mismatch for fixture {fixture_name}");
}

#[test]
fn round_trip_basic_markdown() {
    assert_matches_golden("basic");
}

#[test]
fn round_trip_table_markdown() {
    assert_matches_golden("table");
}

