# No Hardcoding Guide - Configuration & Constants

## Overview

All hardcoded values have been removed and centralized into configuration and constants modules for maintainability and flexibility.

## Modules

### 1. Constants Module (`src/constants.rs`)

Centralized location for all magic numbers and fixed values.

**Categories:**

#### Version Information
```rust
use pptx::constants::version;

println!("Version: {}", version::VERSION_STRING);  // "1.0.2"
```

#### Presentation Defaults
```rust
use pptx::constants::presentation;

let title = presentation::DEFAULT_TITLE;      // "Presentation"
let slides = presentation::DEFAULT_SLIDES;    // 1
let width = presentation::DEFAULT_WIDTH;      // 9144000 EMU
let height = presentation::DEFAULT_HEIGHT;    // 6858000 EMU
```

#### Slide Configuration
```rust
use pptx::constants::slide;

let width = slide::SLIDE_WIDTH;                // 9144000
let height = slide::SLIDE_HEIGHT;              // 6858000
let default_id = slide::DEFAULT_SLIDE_ID;     // 255
```

#### File System Defaults
```rust
use pptx::constants::filesystem;

let dir = filesystem::DEFAULT_OUTPUT_DIR;      // "examples/output"
let ext = filesystem::DEFAULT_EXTENSION;       // "pptx"
let name = filesystem::DEFAULT_FILENAME;       // "presentation"
```

#### XML Configuration
```rust
use pptx::constants::xml;

let version = xml::XML_VERSION;                // "1.0"
let encoding = xml::XML_ENCODING;              // "UTF-8"
let standalone = xml::XML_STANDALONE;          // "yes"
```

#### Theme Configuration
```rust
use pptx::constants::theme;

let theme_name = theme::DEFAULT_THEME_NAME;    // "Office Theme"
let theme_file = theme::DEFAULT_THEME_FILE;    // "theme1"
let color_scheme = theme::DEFAULT_COLOR_SCHEME; // "Office"
let font_scheme = theme::DEFAULT_FONT_SCHEME;   // "Office"
```

#### Layout Configuration
```rust
use pptx::constants::layout;

let layout = layout::DEFAULT_LAYOUT_FILE;      // "slideLayout1"
let master = layout::DEFAULT_MASTER_FILE;      // "slideMaster1"
```

#### Color Palette
```rust
use pptx::constants::colors;

let white = colors::WHITE;                     // "FFFFFF"
let black = colors::BLACK;                     // "000000"
let dark_blue = colors::DARK_BLUE;             // "1F497D"
let light_gray = colors::LIGHT_GRAY;           // "EBEBEB"
let accent_blue = colors::ACCENT_BLUE;         // "4472C4"
let accent_orange = colors::ACCENT_ORANGE;     // "ED7D31"
```

#### Font Configuration
```rust
use pptx::constants::fonts;

let default_font = fonts::DEFAULT_FONT;        // "Calibri"
let major_font = fonts::MAJOR_FONT;            // "Calibri"
let minor_font = fonts::MINOR_FONT;            // "Calibri"
```

#### CLI Configuration
```rust
use pptx::constants::cli;

let app_name = cli::APP_NAME;                  // "pptx-cli"
let author = cli::AUTHOR;                      // "PPTX Generator"
let default_slides = cli::DEFAULT_SLIDES_CLI;  // 1
```

#### Size Constants
```rust
use pptx::constants::sizes;

let kb = sizes::BYTES_PER_KB;                  // 1024
let mb = sizes::BYTES_PER_MB;                  // 1048576
```

#### Relationship IDs
```rust
use pptx::constants::relationships;

let pres_rel = relationships::PRESENTATION_REL_ID;  // "rId1"
let core_rel = relationships::CORE_PROPS_REL_ID;    // "rId2"
let app_rel = relationships::APP_PROPS_REL_ID;      // "rId3"
```

### 2. Config Module (`src/config.rs`)

Runtime configuration for customization.

#### Default Configuration
```rust
use pptx::config::Config;

let config = Config::default();
println!("Output dir: {}", config.default_output_dir);
```

#### Custom Configuration
```rust
use pptx::config::Config;

let config = Config::new(
    5,                    // slides
    "My Presentation",    // title
    "output",             // output_dir
    "pptx"                // extension
);

let path = config.output_path("report");  // "output/report.pptx"
```

#### CLI Configuration
```rust
use pptx::config::CliConfig;

let cli_config = CliConfig::default();
println!("App: {} v{}", cli_config.app_name, cli_config.version);
```

#### Generator Configuration
```rust
use pptx::config::GeneratorConfig;

let gen_config = GeneratorConfig::default();
println!("Theme: {}", gen_config.default_theme);
```

## Usage Examples

### Basic Usage with Defaults
```rust
use pptx::integration::PresentationBuilder;

let builder = PresentationBuilder::new("My Presentation")
    .with_slides(5);
builder.save_to_file("output.pptx")?;
```

### Custom Configuration
```rust
use pptx::integration::PresentationBuilder;
use pptx::config::Config;

let config = Config::new(10, "Report", "reports", "pptx");
let builder = PresentationBuilder::new("My Report")
    .with_config(config)
    .with_slides(10);
builder.save("quarterly_report")?;  // Saves to "reports/quarterly_report.pptx"
```

### Using Constants
```rust
use pptx::constants;

fn create_presentation() {
    let title = constants::presentation::DEFAULT_TITLE;
    let slides = constants::presentation::DEFAULT_SLIDES;
    let width = constants::slide::SLIDE_WIDTH;
    let height = constants::slide::SLIDE_HEIGHT;
    
    println!("Creating: {} ({} slides)", title, slides);
    println!("Size: {} x {}", width, height);
}
```

## Modification Guide

### Adding New Constants

1. **Identify the category** - Does it fit in an existing category?
2. **Add to `src/constants.rs`**:
   ```rust
   pub mod my_category {
       pub const MY_VALUE: &str = "value";
   }
   ```
3. **Use in code**:
   ```rust
   use crate::constants::my_category;
   let value = my_category::MY_VALUE;
   ```

### Changing Configuration

1. **Update `src/config.rs`**:
   ```rust
   pub struct MyConfig {
       pub setting: String,
   }
   ```
2. **Use in builder**:
   ```rust
   let config = MyConfig::default();
   builder.with_config(config)?;
   ```

## Benefits

✅ **Maintainability**
- All values in one place
- Easy to find and update
- Clear organization

✅ **Flexibility**
- Runtime configuration
- Custom settings
- Extensible design

✅ **Type Safety**
- Constants are typed
- Compile-time checking
- No string errors

✅ **Documentation**
- Self-documenting code
- Clear intent
- Easy to understand

✅ **Testing**
- Easy to mock
- Configurable for tests
- Predictable behavior

## Constants Reference

| Constant | Value | Location |
|----------|-------|----------|
| VERSION_STRING | "1.0.2" | `version::VERSION_STRING` |
| DEFAULT_TITLE | "Presentation" | `presentation::DEFAULT_TITLE` |
| DEFAULT_SLIDES | 1 | `presentation::DEFAULT_SLIDES` |
| SLIDE_WIDTH | 9144000 | `slide::SLIDE_WIDTH` |
| SLIDE_HEIGHT | 6858000 | `slide::SLIDE_HEIGHT` |
| DEFAULT_OUTPUT_DIR | "examples/output" | `filesystem::DEFAULT_OUTPUT_DIR` |
| DEFAULT_EXTENSION | "pptx" | `filesystem::DEFAULT_EXTENSION` |
| XML_VERSION | "1.0" | `xml::XML_VERSION` |
| XML_ENCODING | "UTF-8" | `xml::XML_ENCODING` |
| WHITE | "FFFFFF" | `colors::WHITE` |
| BLACK | "000000" | `colors::BLACK` |
| DEFAULT_FONT | "Calibri" | `fonts::DEFAULT_FONT` |
| APP_NAME | "pptx-cli" | `cli::APP_NAME` |

## Best Practices

1. **Use Constants for Fixed Values**
   ```rust
   // ✓ Good
   let width = constants::slide::SLIDE_WIDTH;
   
   // ✗ Bad
   let width = 9144000;
   ```

2. **Use Config for Runtime Settings**
   ```rust
   // ✓ Good
   let config = Config::new(5, "Title", "output", "pptx");
   
   // ✗ Bad
   let output_dir = "output";
   ```

3. **Document Why Values Exist**
   ```rust
   // ✓ Good
   pub const SLIDE_WIDTH: i32 = 9144000;  // EMU (English Metric Units)
   
   // ✗ Bad
   pub const SLIDE_WIDTH: i32 = 9144000;
   ```

4. **Group Related Constants**
   ```rust
   // ✓ Good
   pub mod colors {
       pub const WHITE: &str = "FFFFFF";
       pub const BLACK: &str = "000000";
   }
   
   // ✗ Bad
   pub const COLOR_WHITE: &str = "FFFFFF";
   pub const COLOR_BLACK: &str = "000000";
   ```

## Testing with Constants

```rust
#[cfg(test)]
mod tests {
    use crate::constants;

    #[test]
    fn test_constants() {
        assert_eq!(constants::presentation::DEFAULT_SLIDES, 1);
        assert_eq!(constants::colors::WHITE, "FFFFFF");
    }
}
```

## Conclusion

All hardcoded values have been eliminated through:
- ✅ **Constants Module** - Fixed values
- ✅ **Config Module** - Runtime configuration
- ✅ **Type Safety** - Compile-time checking
- ✅ **Maintainability** - Centralized management
- ✅ **Flexibility** - Easy customization

The codebase is now **hardcode-free** and fully configurable!
