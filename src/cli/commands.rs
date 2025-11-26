//! CLI commands implementation

use std::fs;
use std::path::PathBuf;

pub struct CreateCommand;
pub struct InfoCommand;

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
                    .map_err(|e| format!("Failed to create directory: {}", e))?;
            }
        }

        let title = title.unwrap_or("Presentation");

        // Build presentation metadata
        let mut content = String::new();
        content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        content.push_str("<presentation>\n");
        content.push_str(&format!("  <title>{}</title>\n", escape_xml(title)));
        content.push_str(&format!("  <slides count=\"{}\">\n", slides));

        for i in 1..=slides {
            content.push_str(&format!("    <slide number=\"{}\">\n", i));
            content.push_str(&format!("      <title>Slide {}</title>\n", i));
            content.push_str("      <content></content>\n");
            content.push_str("    </slide>\n");
        }

        content.push_str("  </slides>\n");
        if let Some(tmpl) = template {
            content.push_str(&format!("  <template>{}</template>\n", escape_xml(tmpl)));
        }
        content.push_str("</presentation>\n");

        // Write to file
        fs::write(output, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }
}

impl InfoCommand {
    pub fn execute(file: &str) -> Result<(), String> {
        let metadata = fs::metadata(file)
            .map_err(|e| format!("File not found: {}", e))?;

        let size = metadata.len();
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.elapsed().ok())
            .map(|d| format!("{:?} ago", d))
            .unwrap_or_else(|| "unknown".to_string());

        println!("File Information");
        println!("================");
        println!("Path:     {}", file);
        println!("Size:     {} bytes", size);
        println!("Modified: {}", modified);
        println!("Is file:  {}", metadata.is_file());

        // Try to read and parse as XML
        if let Ok(content) = fs::read_to_string(file) {
            if content.starts_with("<?xml") {
                println!("\nPresentation Information");
                println!("========================");
                if let Some(title_start) = content.find("<title>") {
                    if let Some(title_end) = content[title_start + 7..].find("</title>") {
                        let title = &content[title_start + 7..title_start + 7 + title_end];
                        println!("Title: {}", title);
                    }
                }
                if let Some(slides_start) = content.find("count=\"") {
                    let search_from = slides_start + 7;
                    if let Some(slides_end) = content[search_from..].find("\"") {
                        let count_str = &content[search_from..search_from + slides_end];
                        println!("Slides: {}", count_str);
                    }
                }
            }
        }

        Ok(())
    }
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
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
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }
}
