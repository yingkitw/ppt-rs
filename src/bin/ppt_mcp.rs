//! ppt_mcp - MCP server for ppt-rs
//!
//! Exposes PowerPoint creation and manipulation as MCP tools.
//!
//! ## Usage
//!
//! Run as stdio MCP server:
//!   ppt_mcp
//!
//! Configure in MCP client (e.g. Claude Desktop, Cursor):
//! ```json
//! {
//!   "mcpServers": {
//!     "ppt-rs": {
//!       "command": "ppt_mcp"
//!     }
//!   }
//! }
//! ```

#[tokio::main]
async fn main() {
    if let Err(e) = ppt_rs::mcp::run_server().await {
        eprintln!("ppt_mcp: {}", e);
        std::process::exit(1);
    }
}
