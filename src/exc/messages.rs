//! Unified error message formatting for consistent diagnostics across modules.

/// Missing required package part.
pub fn missing_part(path: &str) -> String {
    format!("Missing required part: {path}")
}

/// Slide index not found in presentation.
pub fn slide_not_found(index: usize) -> String {
    format!("Slide {index} not found")
}

/// Slide part file missing from package.
pub fn slide_file_not_found(path: &str) -> String {
    format!("Slide file not found: {path}")
}

/// Generic index out of range message.
pub fn index_out_of_range(field: &str, index: usize, count: usize) -> String {
    format!("{field} index {index} out of range (count: {count})")
}

/// Field must not be empty.
pub fn must_not_be_empty(field: &str) -> String {
    format!("{field} must not be empty")
}

/// Field must be a positive value.
pub fn must_be_positive(field: &str) -> String {
    format!("{field} must be positive")
}

/// Empty XML document or part.
pub fn empty_xml(path: &str) -> String {
    format!("Empty XML content in '{path}'")
}

/// Empty XML without a specific path.
pub fn empty_xml_content() -> String {
    "Empty XML content".to_string()
}

/// Malformed XML in a specific part.
pub fn invalid_xml(path: &str, detail: &str) -> String {
    format!("Invalid XML in '{path}': {detail}")
}

/// Unsupported file or media format.
pub fn unsupported_format(ext: &str) -> String {
    format!("Unsupported format: {ext}")
}

/// Unsupported media format with extension context.
pub fn unsupported_media_format(ext: &str) -> String {
    format!("Unsupported media format: {ext}")
}

/// Invalid value with field context.
pub fn invalid_value(field: &str, detail: &str) -> String {
    format!("Invalid {field}: {detail}")
}

/// Operation not supported for this element type.
pub fn unsupported_operation(element: &str, operation: &str) -> String {
    format!("{operation} is not supported for {element}")
}

/// External command execution failure.
pub fn command_failed(command: &str, detail: &str) -> String {
    format!("Failed to execute {command}: {detail}")
}

/// External command returned a non-success status.
pub fn command_unsuccessful(command: &str) -> String {
    format!("{command} failed")
}

/// Expected output file was not produced.
pub fn output_not_found(path: &str) -> String {
    format!("Output file not found: {path}")
}
