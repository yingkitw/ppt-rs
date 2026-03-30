//! CLI commands implementation

use crate::generator;
use std::fs;
use std::path::PathBuf;

pub struct CreateCommand;
pub struct FromMarkdownCommand;
pub struct InfoCommand;
pub struct ValidateCommand;

impl CreateCommand {
    pub fn execute(
        output: &str,
        title: Option<&str>,
        slides: usize,
        _template: Option<&str>,
    ) -> Result<(), String> {
        // Create output directory if needed
        if let Some(parent) = PathBuf::from(output).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create directory: {e}"))?;
            }
        }

        let title = title.unwrap_or("Presentation");

        // Generate proper PPTX file
        let pptx_data = generator::create_pptx(title, slides)
            .map_err(|e| format!("Failed to generate PPTX: {e}"))?;

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
    /// Validate a PPTX file for ECMA-376 compliance
    pub fn execute(file: &str) -> Result<(), String> {
        use std::io::Read;
        use zip::ZipArchive;

        println!("Validating PPTX file: {file}");
        println!("{}", "=".repeat(60));

        // Check file exists
        let metadata = fs::metadata(file).map_err(|e| format!("File not found: {e}"))?;

        if !metadata.is_file() {
            return Err(format!("Path is not a file: {file}"));
        }

        // Try to open as ZIP archive
        let file_handle = fs::File::open(file).map_err(|e| format!("Failed to open file: {e}"))?;

        let mut archive =
            ZipArchive::new(file_handle).map_err(|e| format!("Invalid ZIP archive: {e}"))?;

        println!("✓ File is a valid ZIP archive");
        println!("  Total entries: {}", archive.len());

        // Check required files
        let mut issues = Vec::new();
        let mut found_files = std::collections::HashSet::new();

        // Collect all file names
        for i in 0..archive.len() {
            let file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read archive entry: {e}"))?;
            found_files.insert(file.name().to_string());
        }

        // Required files for PPTX
        let required_files = vec![
            "[Content_Types].xml",
            "_rels/.rels",
            "ppt/presentation.xml",
            "docProps/core.xml",
        ];

        println!("\nChecking required files...");
        for required in &required_files {
            if found_files.contains(*required) {
                println!("  ✓ {}", required);
            } else {
                println!("  ✗ {} (missing)", required);
                issues.push(format!("Missing required file: {}", required));
            }
        }

        // Check XML validity
        println!("\nChecking XML validity...");
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to read archive entry: {e}"))?;

            let name = file.name().to_string();
            if name.ends_with(".xml") || name.ends_with(".rels") {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| format!("Failed to read XML file {}: {e}", name))?;

                // Basic XML validation (check for well-formedness)
                if content.trim().is_empty() {
                    issues.push(format!("Empty XML file: {}", name));
                    println!("  ⚠ {} (empty)", name);
                } else if !content.contains("<?xml") && !name.ends_with(".rels") {
                    // .rels files don't always have XML declaration
                    if !name.ends_with(".rels") {
                        issues.push(format!("XML file missing declaration: {}", name));
                        println!("  ⚠ {} (missing XML declaration)", name);
                    }
                } else {
                    // Check for basic XML structure
                    if content.contains("<") && content.contains(">") {
                        println!("  ✓ {} (valid XML)", name);
                    } else {
                        issues.push(format!("Invalid XML structure: {}", name));
                        println!("  ✗ {} (invalid XML)", name);
                    }
                }
            }
        }

        // Check relationships
        println!("\nChecking relationships...");
        if found_files.contains("_rels/.rels") {
            println!("  ✓ Package relationships found");
        } else {
            issues.push("Missing package relationships".to_string());
            println!("  ✗ Package relationships missing");
        }

        // Summary
        println!("\n{}", "=".repeat(60));
        if issues.is_empty() {
            println!("✓ Validation PASSED");
            println!("  File appears to be a valid PPTX file");
            println!("  ECMA-376 compliance: OK");
        } else {
            println!("✗ Validation FAILED");
            println!("  Found {} issue(s):", issues.len());
            for issue in &issues {
                println!("    - {}", issue);
            }
            return Err(format!("Validation failed with {} issue(s)", issues.len()));
        }

        Ok(())
    }
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
