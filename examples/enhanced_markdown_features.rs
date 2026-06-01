//! Example: Using Enhanced Markdown Features
//!
//! This example demonstrates how to use the new Markdown to PowerPoint capabilities:
//! - Real image handling (local files and URLs)
//! - Task list support (GitHub-style checkboxes)
//! - Enhanced inline formatting (strikethrough, subscript, superscript)
//! - Code blocks and syntax highlighting

use ppt_rs::generator::create_pptx_with_content;
use ppt_rs::cli::markdown::parse;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Markdown with real images
    let markdown_with_images = r#"
# Product Launch Presentation

## Features

![Product Logo](./examples/assets/logo.png)

Our new product includes:
- Advanced image processing
- Real-time collaboration
- Cloud synchronization

![Feature Overview](https://via.placeholder.com/800x400/4A90E2/ffffff?text=Feature+Overview)

## Technical Details

The system uses modern architecture with ~~legacy code~~ completely removed.

```rust
fn process_image(img: &Image) -> Result<Processed> {
    // Advanced processing logic
    Ok(Processed::new(img))
}
```

## Development Roadmap

- [x] Core functionality completed
- [x] Image processing implemented
- [ ] Advanced features in progress
- [ ] Performance optimization planned
- [ ] Documentation updates needed

## Performance Metrics

| Metric | Before | After | Improvement |
|--------|--------|-------|-------------|
| Speed | 100ms | 50ms | 2x faster |
| Memory | 50MB | 30MB | 40% reduction |
| Quality | 85% | 95% | 12% improvement |

"#;

    // Parse the markdown
    let slides = parse(markdown_with_images)?;
    let slide_count = slides.len();

    // Create PowerPoint presentation
    let pptx = create_pptx_with_content("Product Launch", slides)?;

    // Save to file
    fs::write("product_launch.pptx", pptx)?;
    println!("Created product_launch.pptx with {} slides", slide_count);

    // Example 2: Technical documentation with enhanced formatting
    let technical_docs = r#"
# API Documentation

## Authentication

The API supports multiple authentication methods:
- **OAuth 2.0** - Recommended for web applications
- **API Keys** - Simple token-based authentication
- ~~Basic Auth~~ - Deprecated, use OAuth instead

## Code Examples

### Python Example
\```python
import requests

response = requests.get('https://api.example.com/data')
data = response.json()
print(data)
\```

### Rust Example
\```rust
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("https://api.example.com/data")
        .send()
        .await?;
    Ok(())
}
\```

## Implementation Status

- [x] Authentication system
- [x] Data endpoints
- [x] Error handling
- [ ] Rate limiting
- [ ] Webhook support
- [ ] Analytics dashboard

## Quick Reference

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/data | Retrieve data |
| POST | /api/data | Create new data |
| PUT | /api/data/:id | Update data |
| DELETE | /api/data/:id | Remove data |

> Note: All endpoints require authentication except the public documentation endpoint.

"#;

    let slides = parse(technical_docs)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("API Documentation", slides)?;
    fs::write("api_docs.pptx", pptx)?;
    println!("Created api_docs.pptx with {} slides", slide_count);

    // Example 3: Task management presentation
    let task_management = r#"
# Project Status Update

## Sprint Progress

### Completed Features ✅

- [x] User authentication system
- [x] Database schema optimization
- [x] API performance improvements
- [x] Frontend component library

### In Progress 🔄

- [ ] Real-time notification system
- [ ] Advanced search functionality
- [ ] Mobile responsive design
- [ ] Analytics dashboard

### Backlog 📋

- [ ] GraphQL API support
- [ ] WebSocket integration
- [ ] Machine learning pipeline
- [ ] Multi-language support

## Technical Achievements

### Performance Optimization

We've achieved significant improvements:

- **Response time**: Reduced from 500ms to 100ms
- **Database queries**: Optimized by ~~75%~~
- **Memory usage**: Reduced by ~~40%~~
- **Cache hit rate**: Increased from 60% to 95%

### Code Quality

```typescript
// Example of improved error handling
async function fetchUserData(userId: string) {
    try {
        const response = await api.get(`/users/${userId}`);
        return response.data;
    } catch (error) {
        logger.error('Failed to fetch user', { userId, error });
        throw new UserFetchError(userId, error);
    }
}
\```

## Next Steps

1. **Complete notification system** - Target: 2 weeks
2. **Launch search feature** - Target: 3 weeks
3. **Mobile optimization** - Target: 4 weeks
4. **Analytics integration** - Target: 5 weeks

## Team Updates

### Development Team
- Frontend: *3 developers* - **On track**
- Backend: *2 developers* - **Ahead of schedule**
- DevOps: *1 engineer* - **On track**

### Blockers & Issues
- ~~API rate limiting~~ - Resolved
- ~~Database migration~~ - Completed
- ~~Testing infrastructure~~ - Deployed

> Remember to update the project tracker and notify stakeholders of progress.

"#;

    let slides = parse(task_management)?;
    let slide_count = slides.len();
    let pptx = create_pptx_with_content("Project Status Update", slides)?;
    fs::write("project_status.pptx", pptx)?;
    println!("Created project_status.pptx with {} slides", slide_count);

    println!("\n✅ All examples created successfully!");
    println!("The new features include:");
    println!("  • Real image downloading from URLs");
    println!("  • Local image file support");
    println!("  • Task list with checkboxes");
    println!("  • Enhanced text formatting (strikethrough, etc.)");
    println!("  • Improved code block handling");
    println!("  • Better table rendering");

    Ok(())
}