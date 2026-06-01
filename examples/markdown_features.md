# Advanced Markdown Presentation Example

This presentation demonstrates the new Markdown to PowerPoint capabilities including real image handling, task lists, and enhanced formatting.

## Features Demonstrated

### 1. Real Image Handling
- **Local Images**: Images from file system
- **Web Images**: Images from URLs  
- **Auto-detection**: PNG, JPEG, GIF, WEBP, SVG

### 2. Enhanced Formatting
- **Strikethrough**: ~~deleted text~~
- **Bold**: **important text**
- **Italic**: *emphasized text*
- **Code**: `inline code`

### 3. Task Lists
- [x] Completed feature
- [ ] Pending task
- [ ] Another todo item

### 4. Code Blocks
```rust
fn main() {
    println!("Hello, PowerPoint!");
}
```

### 5. Tables
| Feature | Status | Priority |
|---------|--------|----------|
| Images | ✅ Done | High |
| Task Lists | ✅ Done | Medium |
| Export | ✅ Done | High |
| CSS | ✅ Done | Medium |

---

![Local Image Example](./examples/assets/sample_image.png)

![Web Image Example](https://via.placeholder.com/600x400/4A90E2/ffffff?text=Sample+Image)

## Technical Implementation

The Markdown parser now includes:

- **Image downloading**: Automatic fetching from URLs
- **Format detection**: Magic byte identification
- **Fallback handling**: Placeholders for failed loads
- **Task support**: Checkbox syntax recognition
- **Enhanced formatting**: Strikethrough and more

## Code Examples

### Enhanced Inline Formatting
This text has **bold**, *italic*, and ~~strikethrough~~ formatting.

### Task Lists
Here are our development tasks:
- [x] Implement image downloading
- [x] Add task list support  
- [x] Enhance CSS parsing
- [ ] Create comprehensive tests
- [ ] Write documentation

## Summary

These new capabilities make ppt-rs significantly more powerful for converting real-world Markdown content into professional PowerPoint presentations.