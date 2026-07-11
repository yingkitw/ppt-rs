//! CLI commands implementation

use crate::generator;
use std::fs;
use std::path::PathBuf;

pub struct CreateCommand;
pub struct FromMarkdownCommand;
pub struct FromHtmlCommand;
pub struct InfoCommand;
pub struct ValidateCommand;

impl CreateCommand {
    pub fn execute(
        output: &str,
        title: Option<&str>,
        slides: usize,
        template: Option<&str>,
    ) -> Result<(), String> {
        // Create output directory if needed
        if let Some(parent) = PathBuf::from(output).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {e}"))?;
            }
        }

        let title = title.unwrap_or("Presentation");

        let pptx_data = if let Some(template_path) = template {
            let slide_list: Vec<_> = (1..=slides)
                .map(|i| generator::SlideContent::new(&format!("Slide {i}")).add_bullet("Content"))
                .collect();
            let settings = generator::PresentationSettings::new().template(template_path);
            generator::create_pptx_with_settings(title, &slide_list, Some(settings))
                .map_err(|e| format!("Failed to generate PPTX from template: {e}"))?
        } else {
            generator::create_pptx(title, slides)
                .map_err(|e| format!("Failed to generate PPTX: {e}"))?
        };

        // Write to file
        fs::write(output, pptx_data).map_err(|e| format!("Failed to write file: {e}"))?;

        Ok(())
    }
}

impl FromMarkdownCommand {
    pub fn execute(input: &str, output: &str, title: Option<&str>) -> Result<(), String> {
        // Read markdown file
        let md_content =
            fs::read_to_string(input).map_err(|e| format!("Failed to read markdown file: {e}"))?;

        // Parse markdown into slides using enhanced parser
        let slides = super::markdown::parse_markdown(&md_content)?;

        if slides.is_empty() {
            return Err("No slides found in markdown file".to_string());
        }

        // Create output directory if needed
        if let Some(parent) = PathBuf::from(output).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {e}"))?;
            }
        }

        let title = title.unwrap_or("Presentation from Markdown");

        // Generate PPTX with content
        let pptx_data = generator::create_pptx_with_content(title, slides)
            .map_err(|e| format!("Failed to generate PPTX: {e}"))?;

        // Write to file
        fs::write(output, pptx_data).map_err(|e| format!("Failed to write file: {e}"))?;

        Ok(())
    }
}

impl FromHtmlCommand {
    pub fn execute(
        input: &str,
        output: &str,
        title: Option<&str>,
        max_slides: usize,
        max_bullets: usize,
        no_images: bool,
        no_tables: bool,
        no_code: bool,
    ) -> Result<(), String> {
        // Read HTML file
        let html_content =
            std::fs::read_to_string(input).map_err(|e| format!("Failed to read HTML file: {e}"))?;

        // Build options
        let options = crate::import::HtmlParseOptions::new()
            .max_slides(max_slides)
            .max_bullets(max_bullets)
            .include_images(!no_images)
            .include_tables(!no_tables)
            .include_code(!no_code);

        // Parse HTML into slides
        let slides = crate::import::parse_html_with_options(&html_content, options)?;

        if slides.is_empty() {
            return Err("No slides found in HTML file".to_string());
        }

        // Create output directory if needed
        if let Some(parent) = PathBuf::from(output).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {e}"))?;
            }
        }

        let title = title.unwrap_or("Presentation from HTML");

        // Generate PPTX with content
        let pptx_data = generator::create_pptx_with_content(title, slides)
            .map_err(|e| format!("Failed to generate PPTX: {e}"))?;

        // Write to file
        fs::write(output, pptx_data).map_err(|e| format!("Failed to write file: {e}"))?;

        Ok(())
    }
}

impl InfoCommand {
    pub fn execute(file: &str) -> Result<(), String> {
        let metadata = fs::metadata(file).map_err(|e| format!("File not found: {e}"))?;

        let size = metadata.len();
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.elapsed().ok())
            .map(|d| format!("{d:?} ago"))
            .unwrap_or_else(|| "unknown".to_string());

        println!("File Information");
        println!("================");
        println!("Path:     {file}");
        println!("Size:     {size} bytes");
        println!("Modified: {modified}");
        let is_file = metadata.is_file();
        println!("Is file:  {is_file}");

        // Try to read and parse as XML
        if let Ok(content) = fs::read_to_string(file) {
            if content.starts_with("<?xml") {
                println!("\nPresentation Information");
                println!("========================");
                if let Some(title_start) = content.find("<title>") {
                    if let Some(title_end) = content[title_start + 7..].find("</title>") {
                        let title = &content[title_start + 7..title_start + 7 + title_end];
                        println!("Title: {title}");
                    }
                }
                if let Some(slides_start) = content.find("count=\"") {
                    let search_from = slides_start + 7;
                    if let Some(slides_end) = content[search_from..].find("\"") {
                        let count_str = &content[search_from..search_from + slides_end];
                        println!("Slides: {count_str}");
                    }
                }
            }
        }

        Ok(())
    }
}

impl ValidateCommand {
    /// Validate a PPTX file and report structured package issues.
    pub fn execute(file: &str, json: bool) -> Result<(), String> {
        let bytes = fs::read(file).map_err(|e| format!("Failed to read file: {e}"))?;
        let report = crate::core::validate_package_bytes(&bytes);

        if json {
            Self::print_json(file, &report);
        } else {
            Self::print_human(file, &report);
        }

        if report.is_valid() {
            Ok(())
        } else {
            Err(format!(
                "Validation failed with {} error(s), {} warning(s)",
                report.error_count(),
                report.warning_count()
            ))
        }
    }

    fn print_human(file: &str, report: &crate::core::PackageValidationReport) {
        use crate::core::ValidationSeverity;

        println!("Validating PPTX file: {file}");
        println!("{}", "=".repeat(60));
        println!(
            "Summary: {} error(s), {} warning(s)",
            report.error_count(),
            report.warning_count()
        );

        if report.issues.is_empty() {
            println!("\n✓ Validation PASSED — no issues found");
            return;
        }

        println!();
        for issue in &report.issues {
            let severity = match issue.severity {
                ValidationSeverity::Error => "ERROR",
                ValidationSeverity::Warning => "WARNING",
            };
            let path = issue.path.as_deref().unwrap_or("-");
            println!(
                "[{severity}] {:?}  {path}",
                issue.category
            );
            println!("  {}", issue.message);
        }

        if report.is_valid() {
            println!("\n✓ Validation PASSED (warnings only)");
        } else {
            println!("\n✗ Validation FAILED");
        }
    }

    fn print_json(file: &str, report: &crate::core::PackageValidationReport) {
        use crate::core::ValidationSeverity;

        println!("{{");
        println!("  \"file\": {},", json_string(file));
        println!("  \"valid\": {},", report.is_valid());
        println!("  \"error_count\": {},", report.error_count());
        println!("  \"warning_count\": {},", report.warning_count());
        println!("  \"issues\": [");
        for (i, issue) in report.issues.iter().enumerate() {
            let severity = match issue.severity {
                ValidationSeverity::Error => "error",
                ValidationSeverity::Warning => "warning",
            };
            let path = issue
                .path
                .as_deref()
                .map(json_string)
                .unwrap_or_else(|| "null".to_string());
            let comma = if i + 1 < report.issues.len() { "," } else { "" };
            println!("    {{");
            println!("      \"severity\": \"{severity}\",");
            println!("      \"category\": \"{:?}\",", issue.category);
            println!("      \"path\": {path},");
            println!("      \"message\": {}", json_string(&issue.message));
            println!("    }}{comma}");
        }
        println!("  ]");
        println!("}}");
    }
}

fn json_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_create_command() {
        let output = "/tmp/test_presentation.pptx";
        let result = CreateCommand::execute(output, Some("Test"), 3, None);
        assert!(result.is_ok());
        assert!(Path::new(output).exists());

        // Cleanup
        let _ = fs::remove_file(output);
    }

    #[test]
    fn test_escape_xml() {
        use crate::core::escape_xml;
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }
}
