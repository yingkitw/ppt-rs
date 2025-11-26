# Examples Summary - PPTX Presentation Generation

## Overview

6 comprehensive examples demonstrating different approaches to generating PowerPoint presentations using Rust.

## Quick Start

### Run all examples
```bash
cd /Users/yingkitw/Desktop/myproject/ppt-rs3
cargo run --examples
```

### Run individual examples
```bash
cargo run --example simple_presentation
cargo run --example multi_slide_presentation
cargo run --example report_generator
cargo run --example batch_generator
cargo run --example training_material
cargo run --example data_driven
```

## Examples Overview

| # | Name | Purpose | Slides | Size | Time |
|---|------|---------|--------|------|------|
| 1 | Simple | Basic presentation | 1 | 233 B | < 1ms |
| 2 | Multi-Slide | Multiple slides | 5 | 806 B | < 1ms |
| 3 | Report | Business report | 6 | 1.0 KB | < 1ms |
| 4 | Batch | Multiple files | 5×(6-12) | 5.7 KB | < 5ms |
| 5 | Training | Course material | 10 | 1.6 KB | < 1ms |
| 6 | Data-Driven | From data | 7 | 1.1 KB | < 1ms |

## Example 1: Simple Presentation

**File:** `examples/simple_presentation.rs` (50 lines)

**Purpose:** Minimal example showing basic presentation creation

**Key Concepts:**
- XML generation
- File I/O
- Directory creation

**Output:**
```
✓ Presentation created: examples/output/simple.pptx
  Title: My First Presentation
  Slides: 1
  Size: 233 bytes
```

**Use Cases:**
- Quick testing
- Minimal requirements
- Learning basics

---

## Example 2: Multi-Slide Presentation

**File:** `examples/multi_slide_presentation.rs` (80 lines)

**Purpose:** Create presentations with multiple slides and content

**Key Concepts:**
- Slide structure
- Content management
- Struct-based design
- Formatted output

**Output:**
```
✓ Presentation created: examples/output/multi_slide.pptx
  Title: Rust PowerPoint Demo
  Slides: 5
  Size: 806 bytes

Slides:
  1. Title Slide
  2. Overview
  3. Features
  4. Benefits
  5. Conclusion
```

**Use Cases:**
- Standard presentations
- Multiple sections
- Organized content

**Slide Structure:**
```
Slide 1: Title Slide
  - Welcome to Rust PowerPoint Generation

Slide 2: Overview
  - This presentation demonstrates PPTX generation in Rust

Slide 3: Features
  - • Create presentations
  - • Add slides
  - • Generate XML

Slide 4: Benefits
  - • Type-safe
  - • Fast
  - • Memory-efficient

Slide 5: Conclusion
  - Thank you!
```

---

## Example 3: Report Generator

**File:** `examples/report_generator.rs` (120 lines)

**Purpose:** Generate business reports with structured data

**Key Concepts:**
- Business metrics
- Formatted numbers
- Date/time integration
- Report structure

**Output:**
```
✓ Report generated: examples/output/quarterly_report.pptx
  Title: Q1 2025 Business Report
  Generated: 2025-11-26 21:50:00
  Size: 1.0 KB
```

**Generated Report:**
- Slide 1: Title with quarter and year
- Slide 2: Executive Summary (revenue, growth, team size)
- Slide 3: Financial Performance (revenue, YoY growth)
- Slide 4: Team Metrics (employees, revenue per employee)
- Slide 5: Key Achievements
- Slide 6: Next Quarter Goals

**Data Used:**
```rust
QuarterlyReport {
    quarter: "Q1",
    year: 2025,
    revenue: 1_500_000.0,
    growth: 15.5,
    employees: 150,
}
```

**Use Cases:**
- Quarterly reports
- Financial presentations
- Business metrics

---

## Example 4: Batch Generator

**File:** `examples/batch_generator.rs` (90 lines)

**Purpose:** Generate multiple presentations in batch

**Key Concepts:**
- Loop-based generation
- Batch processing
- Consistent structure
- Progress reporting

**Output:**
```
Batch generating presentations...

Generating 5 presentations:

✓ Sales Report (8 slides, 1.0 KB)
✓ Marketing Overview (10 slides, 1.2 KB)
✓ Engineering Update (12 slides, 1.4 KB)
✓ HR Initiatives (6 slides, 797 B)
✓ Financial Summary (9 slides, 1.1 KB)

✓ All presentations generated successfully!
```

**Generated Files:**
1. `sales_report.pptx` (8 slides, 1.0 KB)
2. `marketing_overview.pptx` (10 slides, 1.2 KB)
3. `engineering_update.pptx` (12 slides, 1.4 KB)
4. `hr_initiatives.pptx` (6 slides, 797 B)
5. `financial_summary.pptx` (9 slides, 1.1 KB)

**Use Cases:**
- Batch operations
- Multiple reports
- Automated generation
- Scheduled jobs

---

## Example 5: Training Material

**File:** `examples/training_material.rs` (140 lines)

**Purpose:** Generate structured training course materials

**Key Concepts:**
- Course structure
- Module organization
- Topic lists
- Hierarchical content

**Output:**
```
✓ Training material generated: examples/output/rust_training.pptx
  Title: Introduction to Rust
  Duration: 8 hours
  Modules: 4
  Size: 1.6 KB
```

**Course Structure:**

**Module 1: Getting Started**
- Installation
- Hello World
- Cargo

**Module 2: Ownership & Borrowing**
- Ownership Rules
- References
- Lifetimes

**Module 3: Error Handling**
- Result Type
- Option Type
- ? Operator

**Module 4: Advanced Topics**
- Traits
- Generics
- Macros

**Slide Breakdown:**
- Slide 1: Title (course name, duration)
- Slide 2: Agenda (all modules)
- Slides 3-10: Module content (intro + topics for each module)
- Slide 11: Thank You

**Use Cases:**
- Training materials
- Educational content
- Course presentations
- Learning resources

---

## Example 6: Data-Driven Presentation

**File:** `examples/data_driven.rs` (140 lines)

**Purpose:** Generate presentations from structured data

**Key Concepts:**
- Data structures
- Dynamic slide generation
- Formatted data display
- Scalable design

**Output:**
```
✓ Data-driven presentation generated: examples/output/data_driven.pptx
  Company: TechCorp Inc.
  Founded: 2015
  Employees: 500
  Offices: 4
  Products: 3
  Size: 1.1 KB
```

**Generated Slides:**

1. **Title Slide**
   - TechCorp Inc.
   - Founded: 2015

2. **Company Overview**
   - Employees: 500
   - Offices: 4
   - Founded: 2015

3. **Global Presence**
   - Offices in: San Francisco, New York, London, Tokyo

4. **Product A**
   - Revenue: $5,000,000
   - Users: 50,000

5. **Product B**
   - Revenue: $3,000,000
   - Users: 30,000

6. **Product C**
   - Revenue: $2,000,000
   - Users: 20,000

7. **Thank You**
   - Visit us at www.example.com

**Data Structures:**
```rust
struct Product {
    name: String,
    revenue: f64,
    users: u32,
}

struct CompanyData {
    name: String,
    founded: u32,
    employees: u32,
    offices: Vec<&'static str>,
    products: Vec<Product>,
}
```

**Use Cases:**
- Company presentations
- Product showcases
- Data visualization
- Dynamic content

---

## Common Patterns

### Pattern 1: Basic XML Generation
```rust
let mut xml = String::new();
xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
xml.push_str("<presentation>\n");
// ... add content ...
xml.push_str("</presentation>\n");
```

### Pattern 2: Struct-Based Approach
```rust
struct MyPresentation { /* fields */ }

impl MyPresentation {
    fn generate_presentation(&self) -> String {
        // XML generation logic
    }
}
```

### Pattern 3: Loop-Based Batch Processing
```rust
for item in items {
    let presentation = generate(item);
    fs::write(filename, presentation)?;
}
```

### Pattern 4: Data-Driven Slides
```rust
for product in &self.products {
    // Generate slide for each product
}
```

---

## File Structure

```
examples/
├── simple_presentation.rs        (50 lines)
├── multi_slide_presentation.rs   (80 lines)
├── report_generator.rs           (120 lines)
├── batch_generator.rs            (90 lines)
├── training_material.rs          (140 lines)
├── data_driven.rs                (140 lines)
└── output/
    ├── simple.pptx               (233 B)
    ├── multi_slide.pptx          (806 B)
    ├── quarterly_report.pptx     (1.0 KB)
    ├── sales_report.pptx         (1.0 KB)
    ├── marketing_overview.pptx   (1.2 KB)
    ├── engineering_update.pptx   (1.4 KB)
    ├── hr_initiatives.pptx       (797 B)
    ├── financial_summary.pptx    (1.1 KB)
    ├── rust_training.pptx        (1.6 KB)
    └── data_driven.pptx          (1.1 KB)
```

---

## Performance Metrics

| Example | Time | Size | Slides |
|---------|------|------|--------|
| Simple | < 1ms | 233 B | 1 |
| Multi-Slide | < 1ms | 806 B | 5 |
| Report | < 1ms | 1.0 KB | 6 |
| Batch (5 files) | < 5ms | 5.7 KB | 41 |
| Training | < 1ms | 1.6 KB | 11 |
| Data-Driven | < 1ms | 1.1 KB | 7 |

**Total:** 10 files, 11.5 KB, 71 slides, < 10ms

---

## Key Features Demonstrated

✅ **XML Generation**
- Proper escaping
- Hierarchical structure
- Dynamic content

✅ **File I/O**
- Directory creation
- File writing
- Error handling

✅ **Data Structures**
- Structs and enums
- Vectors and collections
- Type safety

✅ **String Formatting**
- Number formatting
- String interpolation
- Content escaping

✅ **Batch Processing**
- Loop-based generation
- Progress reporting
- Multiple outputs

✅ **Error Handling**
- Result types
- Error propagation
- User-friendly messages

---

## Learning Path

1. **Start with Simple**
   - Understand basic structure
   - Learn XML generation
   - Try file I/O

2. **Move to Multi-Slide**
   - Add multiple slides
   - Organize content
   - Use structs

3. **Try Report Generator**
   - Work with data
   - Format numbers
   - Create business content

4. **Explore Batch Generator**
   - Process multiple items
   - Report progress
   - Automate generation

5. **Study Training Material**
   - Organize hierarchical content
   - Create complex structures
   - Build educational content

6. **Master Data-Driven**
   - Generate from data
   - Scale to many items
   - Create dynamic presentations

---

## Tips & Best Practices

1. **Always escape XML** - Use `escape_xml()` for all user content
2. **Create directories** - Use `fs::create_dir_all()` before writing
3. **Use structs** - Organize data with clear types
4. **Handle errors** - Use `Result<T, E>` and `?` operator
5. **Format output** - Provide clear feedback to users
6. **Test thoroughly** - Verify generated files

---

## Next Steps

- Read [EXAMPLES.md](EXAMPLES.md) for detailed documentation
- Check [CLI_GUIDE.md](CLI_GUIDE.md) for command-line usage
- Review [ARCHITECTURE.md](ARCHITECTURE.md) for design details
- Explore [QUICKSTART.md](QUICKSTART.md) for quick reference

---

## License

MIT License - Same as the PPTX library
