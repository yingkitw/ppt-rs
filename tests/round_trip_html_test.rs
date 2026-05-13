//! Round-trip tests for HTML to PPTX conversion
//!
//! Each test reads an HTML fixture, parses it into slides, generates a PPTX,
//! imports the PPTX back, fingerprints the result, and compares to a golden file.

use ppt_rs::api::Presentation;
use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::import::parse_html;

fn fingerprint(pres: &Presentation) -> String {
    let mut out = String::new();

    for (idx, slide) in pres.slides().iter().enumerate() {
        let slide_no = idx + 1;
        out.push_str(&format!("slide: {slide_no}\n"));
        out.push_str(&format!("title: {}\n", slide.title));
        out.push_str("bullets:\n");
        for b in &slide.content {
            out.push_str(&format!("- {b}\n"));
        }

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
        }

        if !slide.code_blocks.is_empty() {
            out.push_str("code:\n");
            for (ci, cb) in slide.code_blocks.iter().enumerate() {
                if ci > 0 {
                    out.push_str("---\n");
                }
                for line in cb.code.lines() {
                    out.push_str(&format!("  {line}\n"));
                }
            }
        }

        if let Some(notes) = &slide.notes {
            out.push_str(&format!("notes: {notes}\n"));
        }

        out.push('\n');
    }

    out
}

fn round_trip_html_fixture(fixture_name: &str) -> String {
    let html_path = format!("tests/fixtures/html/{fixture_name}.html");
    let html = std::fs::read_to_string(&html_path)
        .unwrap_or_else(|e| panic!("failed to read {html_path}: {e}"));

    let slides = parse_html(&html)
        .unwrap_or_else(|e| panic!("parse_html failed for {html_path}: {e}"));

    let pptx_bytes = create_pptx_with_content("Round Trip Test", slides)
        .unwrap_or_else(|e| panic!("create_pptx_with_content failed: {e}"));

    let tmp_path = std::env::temp_dir()
        .join(format!("ppt_rs_round_trip_html_{}.pptx", uuid::Uuid::new_v4()));
    std::fs::write(&tmp_path, &pptx_bytes)
        .unwrap_or_else(|e| panic!("failed to write temp pptx {}: {e}", tmp_path.display()));

    let imported = Presentation::from_path(&tmp_path)
        .unwrap_or_else(|e| panic!("import failed for {}: {e}", tmp_path.display()));

    // Best-effort cleanup
    let _ = std::fs::remove_file(&tmp_path);

    fingerprint(&imported)
}

fn assert_matches_golden(fixture_name: &str) {
    let actual = round_trip_html_fixture(fixture_name);

    let golden_path = format!("tests/goldens/html/{fixture_name}.txt");
    let expected = match std::fs::read_to_string(&golden_path) {
        Ok(content) => content,
        Err(_) => {
            // Auto-generate golden file if it doesn't exist
            eprintln!("Golden file missing, creating: {golden_path}");
            std::fs::write(&golden_path, &actual)
                .unwrap_or_else(|e| panic!("failed to write golden {golden_path}: {e}"));
            actual.clone()
        }
    };

    assert_eq!(actual, expected, "round-trip mismatch for fixture {fixture_name}");
}

#[test]
fn round_trip_basic_html() {
    assert_matches_golden("basic");
}

#[test]
fn round_trip_table_html() {
    assert_matches_golden("table");
}

#[test]
fn round_trip_code_html() {
    assert_matches_golden("code");
}

#[test]
fn round_trip_complex_html() {
    assert_matches_golden("complex");
}

#[test]
fn round_trip_comprehensive_html() {
    assert_matches_golden("comprehensive");
}
