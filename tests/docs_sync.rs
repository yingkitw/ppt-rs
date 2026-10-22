//! Doc-sync checks: make documentation drift a `cargo test` failure.
//!
//! When these tests fail, update the public module/MCP documentation or the
//! TODO.md test counts — not this file.

use std::fs;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("cannot read {rel}: {e}"))
}

fn count_tests_in(dir: &Path) -> usize {
    let mut count = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in fs::read_dir(&d).unwrap() {
            let p = entry.unwrap().path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "rs") {
                // Exclude this file from its own gate — otherwise adding a new
                // drift-detection test would itself drift the integration count.
                if p.ends_with("tests/docs_sync.rs") {
                    continue;
                }
                count += fs::read_to_string(&p)
                    .unwrap()
                    .lines()
                    .filter(|l| l.trim_start().starts_with("#[test]"))
                    .count();
            }
        }
    }
    count
}

/// Extract the integer immediately before `marker` (e.g. "732 inline" → 732).
fn leading_int(text: &str, marker: &str) -> usize {
    let idx = text
        .find(marker)
        .unwrap_or_else(|| panic!("doc missing marker '{marker}'"));
    let digits: String = text[..idx]
        .chars()
        .rev()
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or_else(|_| panic!("no integer before '{marker}'"))
}

#[test]
fn module_organization_documents_every_public_module() {
    let lib = read("src/lib.rs");
    let marker = "//! # Module Organization";
    let start = lib
        .find(marker)
        .unwrap_or_else(|| panic!("src/lib.rs missing `{marker}` section"));
    let module_docs = lib[start..]
        .lines()
        .take_while(|line| line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    let missing: Vec<String> = lib
        .lines()
        .filter_map(|l| {
            let t = l.trim();
            t.strip_prefix("pub mod ")?
                .strip_suffix(';')
                .map(str::to_owned)
        })
        .filter(|m| !module_docs.contains(m))
        .collect();
    assert!(
        missing.is_empty(),
        "public modules missing from the Module Organization rustdoc: {missing:?}"
    );
}

#[test]
fn architecture_md_mentions_every_module() {
    let lib = read("src/lib.rs");
    let arch = read("ARCHITECTURE.md");
    let missing: Vec<String> = lib
        .lines()
        .filter_map(|l| {
            let t = l.trim();
            t.strip_prefix("pub mod ")?
                .strip_suffix(';')
                .map(str::to_owned)
        })
        .filter(|m| !arch.contains(m) && !arch.contains(&format!("{m}::")))
        .collect();
    assert!(
        missing.is_empty(),
        "modules missing from ARCHITECTURE.md: {missing:?}"
    );
}

#[test]
fn todo_md_inline_test_count_matches_code() {
    let actual = count_tests_in(&root().join("src"));
    let claimed = leading_int(&read("TODO.md"), "inline /");
    assert_eq!(
        claimed, actual,
        "TODO.md inline-test count is stale (run `cargo test`, then update the header)"
    );
}

#[test]
fn todo_md_integration_test_count_matches_code() {
    let actual = count_tests_in(&root().join("tests"));
    let claimed = leading_int(&read("TODO.md"), "integration =");
    assert_eq!(
        claimed, actual,
        "TODO.md integration-test count is stale (run `cargo test`, then update the header)"
    );
}

/// Extract the `pub const VERSION` string literal from `src/lib.rs`.
fn lib_version() -> String {
    let lib = read("src/lib.rs");
    let marker = "pub const VERSION: &str = \"";
    let start = lib
        .find(marker)
        .unwrap_or_else(|| panic!("src/lib.rs missing `pub const VERSION`"))
        + marker.len();
    let rest = &lib[start..];
    let end = rest
        .find('"')
        .unwrap_or_else(|| panic!("src/lib.rs VERSION literal not terminated"));
    rest[..end].to_owned()
}

/// Pull every `ppt-rs = "X.Y.Z"` (or `ppt-rs = { version = "X.Y.Z", … }`) from README.
fn readme_install_versions() -> Vec<String> {
    let readme = read("README.md");
    let mut out = Vec::new();
    let mut rest = readme.as_str();
    while let Some(idx) = rest.find("ppt-rs = ") {
        let after = &rest[idx + "ppt-rs = ".len()..];
        let after = after.trim_start();
        if let Some(stripped) = after.strip_prefix('"')
            && let Some(end) = stripped.find('"')
        {
            out.push(stripped[..end].to_owned());
            rest = &rest[idx + "ppt-rs = ".len() + (after.len() - stripped.len()) + end + 1..];
            continue;
        }
        if let Some(stripped) = after.strip_prefix('{')
            && let Some(v_idx) = stripped.find("version = \"")
        {
            let v_after = &stripped[v_idx + "version = \"".len()..];
            if let Some(end) = v_after.find('"') {
                out.push(v_after[..end].to_owned());
                rest = &rest[idx + "ppt-rs = ".len() + (after.len() - stripped.len()) + end + 1..];
                continue;
            }
        }
        rest = &rest[idx + "ppt-rs = ".len()..];
    }
    out
}

#[test]
fn readme_install_versions_match_lib_version() {
    let actual = lib_version();
    let claimed = readme_install_versions();
    assert!(
        !claimed.is_empty(),
        "README.md does not document an install version (expected `ppt-rs = \"{actual}\"`)"
    );
    assert!(
        claimed.iter().all(|v| v == &actual),
        "README install snippet stale: lib = {actual}, README mentions = {claimed:?}"
    );
}

/// Count `#[tool]`-annotated methods on `PptMcp` — i.e. methods declared as
/// `fn <ident>(` whose immediately preceding attribute block contains
/// `#[tool(…)]`. The MCP tool list is documented in the README table.
fn mcp_tool_count() -> usize {
    let mcp = read("src/mcp/mod.rs");
    let mut count = 0;
    // State we carry across lines:
    let mut attr_depth: i32 = 0; // bracket nesting inside the current `#[…]`
    let mut pending_tool = false; // last `#[…]` we saw started with `#[tool`
    for raw in mcp.lines() {
        let line = raw.trim_start();
        // Track whether we are inside a multi-line `#[ … ]` attribute block so
        // body lines like `name = "…"`, `description = "…"` don't reset the
        // pending_tool flag.
        if attr_depth > 0 {
            for ch in line.chars() {
                match ch {
                    '[' => attr_depth += 1,
                    ']' => attr_depth -= 1,
                    _ => {}
                }
            }
            continue;
        }
        if line.starts_with("#[tool") {
            pending_tool = true;
            // Count any further `[`/`]` on the same line (rare for `#[tool(`).
            for ch in line.chars() {
                match ch {
                    '[' => attr_depth += 1,
                    ']' => attr_depth -= 1,
                    _ => {}
                }
            }
            continue;
        }
        if line.starts_with('#') {
            pending_tool = false;
            continue;
        }
        if pending_tool && line.starts_with("fn ") && line.contains('(') && !line.contains("::") {
            count += 1;
            pending_tool = false;
            continue;
        }
        // Anything else (statement, comment, blank) clears the flag — a tool
        // attribute is consumed by the very next `fn`.
        if !line.is_empty() && !line.starts_with("//") {
            pending_tool = false;
        }
    }
    count
}

fn readme_mcp_tool_count() -> usize {
    let readme = read("README.md");
    let section = readme
        .split_once("### Exposed tools")
        .map(|(_, section)| section)
        .unwrap_or_else(|| panic!("README.md missing `### Exposed tools` section"));
    let mut table_started = false;
    let mut names = Vec::new();
    for line in section.lines().map(str::trim) {
        if !table_started {
            if line.starts_with("| Tool |") {
                table_started = true;
            }
            continue;
        }
        if !line.starts_with('|') {
            break;
        }
        if let Some(entry) = line.strip_prefix("| `") {
            let end = entry
                .find('`')
                .unwrap_or_else(|| panic!("malformed README MCP tool row: {line}"));
            names.push(&entry[..end]);
        }
    }
    assert!(table_started, "README MCP tools table header is missing");
    assert!(!names.is_empty(), "README MCP tools table is empty");
    let mut unique = names.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(
        unique.len(),
        names.len(),
        "README MCP tools table contains duplicate tool names"
    );
    names.len()
}

#[test]
fn mcp_tools_are_documented_consistently() {
    let actual = mcp_tool_count();
    assert!(
        actual > 0,
        "could not detect any MCP tools in src/mcp/mod.rs — tool-marking convention changed?"
    );
    let documented = readme_mcp_tool_count();
    assert_eq!(
        documented, actual,
        "README MCP tools table stale: code = {actual}, README lists = {documented}"
    );
}

/// Extract the `pub enum Commands` variant names from `src/cli/parser.rs`.
fn cli_command_variants() -> Vec<String> {
    let parser = read("src/cli/parser.rs");
    let start = parser
        .find("pub enum Commands")
        .unwrap_or_else(|| panic!("src/cli/parser.rs missing `pub enum Commands`"));
    let rest = &parser[start..];
    let mut variants = Vec::new();
    // `depth` counts `{` minus `}` so we can detect when we are inside the
    // enum body (depth == 1) versus inside a variant body (depth == 2).
    let mut depth: i32 = 0;
    let mut entered = false;
    for raw in rest.lines() {
        let line = raw;
        // Decide BEFORE updating `depth` whether this line opens a variant:
        // a variant declaration is exactly `    <Name> {` while the enum is
        // open (depth == 1 before the `{` on this line).
        let trimmed = line.trim_start();
        if entered
            && depth == 1
            && !trimmed.is_empty()
            && !trimmed.starts_with("//")
            && !trimmed.starts_with('#')
            && let Some(t) = trimmed.strip_suffix('{')
        {
            let ident = t.split_whitespace().next().unwrap_or("");
            if !ident.is_empty() {
                let first = ident.chars().next().unwrap();
                if first.is_ascii_uppercase() {
                    variants.push(ident.to_owned());
                }
            }
        }
        // Now update depth for braces on this line (the variant's `{` pushes
        // us to depth == 2 for its body lines).
        for ch in line.chars() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if entered && depth == 0 {
                        return variants;
                    }
                }
                _ => {}
            }
        }
        // The enum header `pub enum Commands {` opens the body. Mark
        // `entered` once we've consumed its `{`.
        if !entered && depth == 1 {
            entered = true;
        }
    }
    variants
}

#[test]
fn cli_command_set_matches_module_summary() {
    let variants = cli_command_variants();
    assert!(
        variants.len() >= 2,
        "could not detect any CLI command variants in src/cli/parser.rs — parser shape changed? got {variants:?}"
    );
    // Cross-check: ARCHITECTURE.md / README.md must reference every CLI variant
    // somewhere. Users invoke the lowercase form (`pptcli md2ppt`), so we
    // match case-insensitively — drift surfaces if a variant is renamed or
    // removed without updating either root doc.
    let arch = read("ARCHITECTURE.md").to_lowercase();
    let readme = read("README.md").to_lowercase();
    let missing: Vec<&String> = variants
        .iter()
        .filter(|v| {
            let needle = v.to_lowercase();
            !arch.contains(&needle) && !readme.contains(&needle)
        })
        .collect();
    assert!(
        missing.is_empty(),
        "CLI variants missing from ARCHITECTURE.md / README.md: {missing:?} (rename or add a doc mention)"
    );
}
