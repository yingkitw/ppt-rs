//! Web2PPT Demo - Convert webpages to PowerPoint
//!
//! Run with: cargo run --example web2ppt_demo --features web2ppt

#[cfg(feature = "web2ppt")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ppt_rs::{
        html_to_pptx, html_to_pptx_with_options,
        Web2PptConfig, ConversionOptions,
    };

    println!("=== Web2PPT Demo ===\n");

    // Example 1: Convert HTML string to PPTX
    println!("📄 Example 1: HTML to PPTX");
    
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Rust Programming Language</title>
            <meta name="description" content="A systems programming language focused on safety and performance">
        </head>
        <body>
            <main>
                <h1>Rust Programming Language</h1>
                <p>Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency.</p>
                
                <h2>Key Features</h2>
                <ul>
                    <li>Memory safety without garbage collection</li>
                    <li>Concurrency without data races</li>
                    <li>Zero-cost abstractions</li>
                    <li>Minimal runtime</li>
                    <li>Efficient C bindings</li>
                </ul>
                
                <h2>Getting Started</h2>
                <p>Install Rust using rustup, the official Rust toolchain installer. It manages Rust versions and associated tools.</p>
                
                <pre><code>curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
                
                <h2>Hello World</h2>
                <p>Create your first Rust program with a simple hello world example that demonstrates the basic syntax.</p>
                
                <pre><code>fn main() {
    println!("Hello, world!");
}</code></pre>
                
                <h2>Cargo</h2>
                <p>Cargo is Rust's build system and package manager. It handles downloading dependencies, compiling code, and more.</p>
                <ul>
                    <li>cargo new - Create a new project</li>
                    <li>cargo build - Build your project</li>
                    <li>cargo run - Run your project</li>
                    <li>cargo test - Run tests</li>
                </ul>
                
                <h2>Community</h2>
                <p>Rust has a welcoming and helpful community. Join the official forums, Discord, or Reddit to connect with other Rustaceans.</p>
            </main>
        </body>
        </html>
    "#;

    let pptx = html_to_pptx(html, "https://rust-lang.org")?;
    std::fs::create_dir_all("examples/output")?;
    std::fs::write("examples/output/rust_intro.pptx", &pptx)?;
    println!("   ✅ Created rust_intro.pptx ({} bytes)\n", pptx.len());

    // Example 2: With custom options
    println!("📄 Example 2: Custom options");
    
    let config = Web2PptConfig::new()
        .max_slides(5)
        .max_bullets(4)
        .with_code(true);

    let options = ConversionOptions::new()
        .title("Rust Quick Start")
        .author("ppt-rs")
        .with_source_url(true);

    let pptx = html_to_pptx_with_options(html, "https://rust-lang.org", config, options)?;
    std::fs::write("examples/output/rust_quick.pptx", &pptx)?;
    println!("   ✅ Created rust_quick.pptx ({} bytes)\n", pptx.len());

    // Example 3: Technical documentation style
    println!("📄 Example 3: Technical documentation");
    
    let tech_html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>API Documentation</title></head>
        <body>
            <main>
                <h1>REST API Reference</h1>
                <p>This document describes the REST API endpoints available for integration with our platform.</p>
                
                <h2>Authentication</h2>
                <p>All API requests require authentication using Bearer tokens in the Authorization header.</p>
                <pre><code>Authorization: Bearer YOUR_API_KEY</code></pre>
                
                <h2>Endpoints</h2>
                
                <h3>GET /users</h3>
                <p>Retrieve a list of all users in the system with pagination support.</p>
                <ul>
                    <li>page - Page number (default: 1)</li>
                    <li>limit - Items per page (default: 20)</li>
                    <li>sort - Sort field (name, email, created_at)</li>
                </ul>
                
                <h3>POST /users</h3>
                <p>Create a new user account with the specified details and permissions.</p>
                <pre><code>{
  "name": "John Doe",
  "email": "john@example.com",
  "role": "user"
}</code></pre>
                
                <h3>GET /users/{id}</h3>
                <p>Retrieve details for a specific user by their unique identifier.</p>
                
                <h2>Error Handling</h2>
                <p>The API uses standard HTTP status codes to indicate success or failure of requests.</p>
                <ul>
                    <li>200 - Success</li>
                    <li>400 - Bad Request</li>
                    <li>401 - Unauthorized</li>
                    <li>404 - Not Found</li>
                    <li>500 - Server Error</li>
                </ul>
                
                <h2>Rate Limiting</h2>
                <p>API requests are limited to 100 requests per minute per API key to ensure fair usage.</p>
            </main>
        </body>
        </html>
    "#;

    let pptx = html_to_pptx(tech_html, "https://api.example.com/docs")?;
    std::fs::write("examples/output/api_docs.pptx", &pptx)?;
    println!("   ✅ Created api_docs.pptx ({} bytes)\n", pptx.len());

    println!("=== Demo Complete ===");
    println!("\nGenerated files in examples/output/:");
    println!("  - rust_intro.pptx");
    println!("  - rust_quick.pptx");
    println!("  - api_docs.pptx");

    Ok(())
}

#[cfg(not(feature = "web2ppt"))]
fn main() {
    eprintln!("❌ This example requires the 'web2ppt' feature.");
    eprintln!("   Run with: cargo run --example web2ppt_demo --features web2ppt");
}
