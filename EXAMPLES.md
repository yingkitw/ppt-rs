# PPTX Examples - Usage Demonstrations

This directory contains 6 comprehensive examples demonstrating different ways to generate PowerPoint presentations using the PPTX library.

## Running Examples

### Build all examples
```bash
cargo build --examples
```

### Run a specific example
```bash
cargo run --example simple_presentation
cargo run --example multi_slide_presentation
cargo run --example report_generator
cargo run --example batch_generator
cargo run --example training_material
cargo run --example data_driven
```

## Example 1: Simple Presentation

**File:** `examples/simple_presentation.rs`

**Purpose:** Create a basic single-slide presentation

**Key Features:**
- Minimal setup
- Single slide
- Basic XML generation
- File I/O

**Output:** `examples/output/simple.pptx`

**Run:**
```bash
cargo run --example simple_presentation
```

**Output:**
```
Creating a simple presentation...

✓ Presentation created: examples/output/simple.pptx
  Title: My First Presentation
  Slides: 1
  Size: 234 bytes
```

**Use Case:** Quick presentation generation, testing, minimal requirements

---

## Example 2: Multi-Slide Presentation

**File:** `examples/multi_slide_presentation.rs`

**Purpose:** Create a presentation with multiple slides and content

**Key Features:**
- Slide structure
- Content per slide
- Struct-based design
- Formatted output

**Output:** `examples/output/multi_slide.pptx`

**Run:**
```bash
cargo run --example multi_slide_presentation
```

**Output:**
```
Creating a multi-slide presentation...

✓ Presentation created: examples/output/multi_slide.pptx
  Title: Rust PowerPoint Demo
  Slides: 5
  Size: 456 bytes

Slides:
  1. Title Slide
  2. Overview
  3. Features
  4. Benefits
  5. Conclusion
```

**Use Case:** Standard presentations, multiple sections, organized content

---

## Example 3: Report Generator

**File:** `examples/report_generator.rs`

**Purpose:** Generate business reports with structured data

**Key Features:**
- Business metrics
- Formatted numbers
- Date/time integration
- Report structure
- Multiple sections

**Output:** `examples/output/quarterly_report.pptx`

**Run:**
```bash
cargo run --example report_generator
```

**Output:**
```
Generating quarterly business report...

✓ Report generated: examples/output/quarterly_report.pptx
  Title: Q1 2025 Business Report
  Generated: 2025-11-26 21:50:00
  Size: 789 bytes
```

**Use Case:** Quarterly reports, financial presentations, business metrics

**Generated Slides:**
1. Title Slide
2. Executive Summary
3. Financial Performance
4. Team Metrics
5. Key Achievements
6. Next Quarter Goals

---

## Example 4: Batch Generator

**File:** `examples/batch_generator.rs`

**Purpose:** Generate multiple presentations in batch

**Key Features:**
- Loop-based generation
- Multiple presentations
- Consistent structure
- Batch processing
- Progress reporting

**Output:** 
- `examples/output/sales_report.pptx`
- `examples/output/marketing_overview.pptx`
- `examples/output/engineering_update.pptx`
- `examples/output/hr_initiatives.pptx`
- `examples/output/financial_summary.pptx`

**Run:**
```bash
cargo run --example batch_generator
```

**Output:**
```
Batch generating presentations...

Generating 5 presentations:

✓ Sales Report (8 slides, 456 bytes)
✓ Marketing Overview (10 slides, 567 bytes)
✓ Engineering Update (12 slides, 678 bytes)
✓ HR Initiatives (6 slides, 345 bytes)
✓ Financial Summary (9 slides, 512 bytes)

✓ All presentations generated successfully!
```

**Use Case:** Batch operations, multiple reports, automated generation

---

## Example 5: Training Material

**File:** `examples/training_material.rs`

**Purpose:** Generate structured training course materials

**Key Features:**
- Course structure
- Module organization
- Topic lists
- Hierarchical content
- Educational layout

**Output:** `examples/output/rust_training.pptx`

**Run:**
```bash
cargo run --example training_material
```

**Output:**
```
Generating Rust training material...

✓ Training material generated: examples/output/rust_training.pptx
  Title: Introduction to Rust
  Duration: 8 hours
  Modules: 4
  Size: 1234 bytes
```

**Generated Modules:**
1. Getting Started
   - Installation
   - Hello World
   - Cargo

2. Ownership & Borrowing
   - Ownership Rules
   - References
   - Lifetimes

3. Error Handling
   - Result Type
   - Option Type
   - ? Operator

4. Advanced Topics
   - Traits
   - Generics
   - Macros

**Use Case:** Training materials, educational content, course presentations

---

## Example 6: Data-Driven Presentation

**File:** `examples/data_driven.rs`

**Purpose:** Generate presentations from structured data

**Key Features:**
- Data structures
- Dynamic slide generation
- Formatted data display
- Scalable design
- Real-world data

**Output:** `examples/output/data_driven.pptx`

**Run:**
```bash
cargo run --example data_driven
```

**Output:**
```
Generating data-driven presentation...

✓ Data-driven presentation generated: examples/output/data_driven.pptx
  Company: TechCorp Inc.
  Founded: 2015
  Employees: 500
  Offices: 4
  Products: 3
  Size: 890 bytes
```

**Generated Slides:**
1. Title Slide (Company name, founding year)
2. Company Overview (Employees, offices, founding year)
3. Global Presence (Office locations)
4. Product A (Revenue, users)
5. Product B (Revenue, users)
6. Product C (Revenue, users)
7. Thank You (Contact info)

**Use Case:** Company presentations, product showcases, data visualization

---

## Common Patterns

### Pattern 1: Basic Structure
```rust
let presentation = create_presentation(title, slides);
fs::write(output_file, presentation)?;
```

### Pattern 2: Struct-Based
```rust
struct MyPresentation {
    // fields
}

impl MyPresentation {
    fn generate_presentation(&self) -> String {
        // XML generation
    }
}
```

### Pattern 3: Data-Driven
```rust
let data = load_data();
let slides = data.into_iter()
    .map(|item| create_slide(item))
    .collect();
```

### Pattern 4: Batch Processing
```rust
for item in items {
    let presentation = generate(item);
    fs::write(filename, presentation)?;
}
```

## XML Escaping

All examples properly escape XML special characters:
```rust
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}
```

## Output Directory

All examples create presentations in `examples/output/`:
```
examples/output/
├── simple.pptx
├── multi_slide.pptx
├── quarterly_report.pptx
├── sales_report.pptx
├── marketing_overview.pptx
├── engineering_update.pptx
├── hr_initiatives.pptx
├── financial_summary.pptx
├── rust_training.pptx
└── data_driven.pptx
```

## Performance

| Example | Time | Size |
|---------|------|------|
| Simple | < 1ms | 234 bytes |
| Multi-Slide | < 1ms | 456 bytes |
| Report | < 1ms | 789 bytes |
| Batch (5 files) | < 5ms | 2.5 KB |
| Training | < 1ms | 1.2 KB |
| Data-Driven | < 1ms | 890 bytes |

## Extending Examples

To create your own example:

1. Create `examples/my_example.rs`
2. Implement main function returning `Result<(), Box<dyn std::error::Error>>`
3. Create output directory
4. Generate presentation XML
5. Write to file
6. Run with `cargo run --example my_example`

## Tips

- Always escape XML content
- Create output directories before writing
- Use meaningful slide titles
- Organize content logically
- Test with various data sizes
- Consider performance for large presentations

## Next Steps

- Explore the [CLI Guide](CLI_GUIDE.md)
- Read the [Architecture](ARCHITECTURE.md)
- Check the [Quick Start](QUICKSTART.md)
- Review the [Translation Progress](TRANSLATION_PROGRESS.md)

## License

MIT License - Same as the PPTX library
