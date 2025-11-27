# TODO - PPTX-RS Development

## Recently Completed

### Code Modularization (v1.0.4)
- [x] Modularized `generator/layouts/` - Atomic slide layout generators
  - `common.rs` - SlideXmlBuilder, generate_text_props
  - `blank.rs` - BlankLayout
  - `title_only.rs` - TitleOnlyLayout
  - `centered_title.rs` - CenteredTitleLayout
  - `title_content.rs` - TitleContentLayout, TitleBigContentLayout
  - `two_column.rs` - TwoColumnLayout
- [x] Modularized `generator/text/` - Atomic text components
  - `format.rs` - TextFormat, FormattedText
  - `run.rs` - Run (text span with formatting)
  - `paragraph.rs` - Paragraph (alignment, bullets, spacing)
  - `frame.rs` - TextFrame (container for paragraphs)
- [x] Modularized `generator/charts/` - Atomic chart components
  - `types.rs` - ChartType enum
  - `data.rs` - Chart, ChartSeries
  - `builder.rs` - ChartBuilder
  - `xml.rs` - Chart XML generation with shared helpers
- [x] All 303 tests passing

### Code Optimization (v1.0.3)
- [x] Fixed failing test (file size check too strict)
- [x] Created core module with trait-based XML generation (`ToXml`, `Positioned`, `Styled`)
- [x] Added `XmlWriter` utility for efficient XML building
- [x] Consolidated `escape_xml` and color handling in `core::xml_utils`
- [x] Removed deprecated stub modules, simplified `lib.rs`
- [x] Implemented `api::Presentation` with builder pattern
- [x] Re-exported all major types from lib.rs for convenience
- [x] Modularized `generator/xml.rs` (1135 lines → 5 modules):
  - `slide_content.rs` (160 lines) - SlideLayout, SlideContent
  - `package_xml.rs` (85 lines) - content types, relationships
  - `slide_xml.rs` (718 lines) - slide layout XML generation
  - `theme_xml.rs` (137 lines) - theme, master, layout XML
  - `props_xml.rs` (42 lines) - document properties
- [x] All tests passing

## Completed

### 1. Basic PPTX Generation ✓
- [x] ZIP file writing with proper structure
- [x] XML generation for all required components
- [x] Proper ECMA-376 compliance
- [x] CLI tool for basic PPTX creation
- [x] Support for custom slide titles

### 2. Complex PPTX Generation ✓
- [x] Slide content with bullet points
- [x] Text formatting (bold titles, regular text)
- [x] Multiple text boxes per slide
- [x] SlideContent builder API
- [x] Examples for business, training, and proposal presentations

### 3. Code Organization ✓
- [x] Modularized generator.rs (620 lines → 3 files)
- [x] Modularized integration.rs (180 lines → 3 files)
- [x] Cleaned up lib.rs with clear module organization
- [x] Marked deprecated/stub modules
- [x] Created CODEBASE.md documentation
- [x] Improved public API exports

### 4. Advanced Content Features ✓
- [x] Text formatting module with builder API
- [x] Font sizing (working: 8pt to 96pt)
- [x] Bold formatting (working)
- [x] Italic formatting (implemented)
- [x] Underline formatting (implemented)
- [x] Text colors (implemented)
- [x] Shapes module with multiple shape types (implemented)
- [x] Tables module with cell and row support (implemented)
- [x] Table XML generation (implemented)
- [x] Table integration into slides (implemented)
- [x] Advanced features example
- [x] Image support with positioning and sizing
- [x] Image XML generation and relationships

## High Priority - Next Steps

### 1. XML Parsing for Reading Presentations ✓
- [x] Implement XML parser in `oxml/xmlchemy.rs` (XmlParser, XmlElement)
- [x] Parse slide content from existing PPTX files (SlideParser)
- [x] Extract text, shapes, tables from slides
- [x] Build object model from XML (ParsedSlide, ParsedShape, TextRun)
- [x] PresentationReader for high-level PPTX reading
- [x] Extract presentation metadata (title, creator, dates)
- [x] Example: `read_presentation.rs`

### 2. Slide Modification Capabilities ✓
- [x] Open existing PPTX files (PresentationReader::open, PresentationEditor::open)
- [x] Parse and modify slide content (PresentationEditor::update_slide)
- [x] Add new slides to existing presentations (PresentationEditor::add_slide)
- [x] Remove slides (PresentationEditor::remove_slide)
- [x] Save modified presentations (PresentationEditor::save)
- [x] Example: `edit_presentation.rs`

### 3. Enhanced Content Integration
- [x] Embed tables directly into slides
- [x] Embed charts directly into slides
- [x] Embed images directly into slides (placeholder)
- [x] Update `create_pptx_with_content` to support rich content

### 4. Slide Layouts ✓
- [x] Implement SlideLayout enum with 6 layout types
- [x] TitleOnly layout
- [x] CenteredTitle layout
- [x] TitleAndContent layout (default)
- [x] TitleAndBigContent layout
- [x] TwoColumn layout (with auto-split bullets)
- [x] Blank layout
- [x] Add layout builder method to SlideContent
- [x] Update slide XML generation for each layout
- [x] Create layout_demo.rs example
- [x] Update README with layout documentation

### 5. Table Rendering ✓
- [x] Add table field to SlideContent struct
- [x] Add table builder method to SlideContent
- [x] Integrate table XML generation into slide rendering
- [x] Tables render instead of bullets when present
- [x] Support styled tables with colors and bold
- [x] Create table_demo.rs example
- [x] Update README with table documentation
- [x] Fix table cell XML generation with proper margins and text anchor
- [x] Create table_generation.rs example with real table data
- [x] Verify all table examples generate valid PPTX files

## Completed Features

### 1. Complete Text Styling ✓
- [x] Implement italic formatting in XML
- [x] Implement underline formatting in XML
- [x] Implement text color in XML
- [x] Update SlideContent to use TextFormat
- [x] Test and verify in PowerPoint

### 2. Table Implementation ✓
- [x] Design table XML structure
- [x] Implement table XML generation
- [x] Implement cell XML generation
- [x] Integrate tables into slide generation
- [x] Test with various table sizes
- [x] Verify in PowerPoint

### 3. Image Implementation ✓
- [x] Design image embedding approach
- [x] Implement image XML generation
- [x] Handle image positioning and sizing
- [x] Integrate images into slide generation
- [x] Test with various image formats

### 4. Chart Implementation ✓
- [x] Design chart XML structure
- [x] Implement chart XML generation
- [x] Support multiple chart types (bar, line, pie)
- [x] Chart builder API with fluent interface
- [x] Test with various data sets
- [x] Example programs demonstrating charts

### 5. Reading & Modification ✓
- [x] ZIP reading in `opc/package.rs` (implemented)
- [x] ZIP writing in `opc/package.rs` (implemented)
- [x] Package part management (get, add, list, remove)
- [x] Example: read_pptx.rs - Read and inspect PPTX files
- [x] SlideContent extended with table, chart, image markers
- [x] Comprehensive demo updated with all feature indicators
- [x] XML parsing in `oxml/xmlchemy.rs` (XmlParser, XmlElement)
- [x] Open existing PPTX files and extract metadata (PresentationReader)
- [x] Modify existing presentations (PresentationEditor)
- [x] Add slides to existing presentations (add_slide)
- [x] Update slides (update_slide)
- [x] Remove slides (remove_slide)
- [x] Examples: read_presentation.rs, edit_presentation.rs

### 4. Parts Implementation ✓
- [x] `parts/base.rs` - Part trait, PartType, ContentType
- [x] `parts/presentation.rs` - PresentationPart
- [x] `parts/slide.rs` - SlidePart
- [x] `parts/image.rs` - ImagePart
- [x] `parts/chart.rs` - ChartPart
- [x] `parts/coreprops.rs` - CorePropertiesPart
- [x] `parts/relationships.rs` - Relationship, Relationships

## Medium Priority

### 4. Shape Implementation ✓
- [x] `generator/shapes.rs` - Shape, ShapeType (40+ types), ShapeFill, ShapeLine
- [x] `generator/shapes_xml.rs` - XML generation for shapes
- [x] Basic shapes (rect, ellipse, triangle, diamond, etc.)
- [x] Arrow shapes (8 directions)
- [x] Stars and banners
- [x] Callouts
- [x] Flow chart shapes
- [x] Fill colors with transparency
- [x] Line/border styling
- [x] Text inside shapes
- [x] Connectors with arrow heads
- [x] Example: `shapes_demo.rs`

### 5. Text Implementation ✓
- [x] `generator/text.rs` - TextFrame, Paragraph, Run
- [x] TextAlign (Left, Center, Right, Justify)
- [x] TextAnchor (Top, Middle, Bottom)
- [x] Run with formatting (bold, italic, underline, color, size, font)
- [x] Paragraph with alignment, bullets, spacing
- [x] TextFrame with margins and anchor
- [x] Font family support

### 6. OXML Element Implementations
- [ ] `oxml/presentation.rs` - Presentation elements
- [ ] `oxml/slide.rs` - Slide elements
- [ ] `oxml/shapes/` - Shape elements
- [ ] `oxml/text.rs` - Text elements
- [ ] `oxml/table.rs` - Table elements
- [ ] `oxml/dml/` - DML elements
- [ ] `oxml/chart/` - Chart elements

## Lower Priority

### 7. Chart Implementation
- [ ] `chart/chart.rs` - Chart
- [ ] `chart/axis.rs` - Axis
- [ ] `chart/series.rs` - Series
- [ ] `chart/plot.rs` - Plot
- [ ] `chart/data.py` - Chart data
- [ ] Implement chart factory

### 8. DML Implementation
- [ ] `dml/color.rs` - Color handling
- [ ] `dml/fill.rs` - Fill handling
- [ ] `dml/line.rs` - Line handling
- [ ] `dml/effect.rs` - Effects

### 9. Table Implementation
- [ ] `table.rs` - Table
- [ ] Implement table cells
- [ ] Implement table rows/columns

### 10. Media & Themes
- [ ] `media.rs` - Media handling
- [ ] `oxml/theme.rs` - Theme elements
- [ ] Implement theme support

## Testing

### Unit Tests
- [ ] Test Length conversions in `util.rs`
- [ ] Test PackUri operations in `opc/packuri.rs`
- [ ] Test enum values in `enums/`
- [ ] Test namespace handling in `oxml/ns.rs`

### Integration Tests
- [ ] Test creating a presentation
- [ ] Test opening a presentation
- [ ] Test adding slides
- [ ] Test adding shapes
- [ ] Test adding text
- [ ] Test saving presentations

### Example Programs
- [ ] Create simple presentation example
- [ ] Create presentation with shapes example
- [ ] Create presentation with text example
- [ ] Create presentation with images example

## Documentation

- [ ] Complete API documentation
- [ ] Add usage examples
- [ ] Add troubleshooting guide

## Performance Optimization

- [ ] Profile memory usage
- [ ] Optimize XML parsing
- [ ] Optimize ZIP operations
- [ ] Consider lazy loading

## Code Quality

- [ ] Fix remaining clippy warnings
- [ ] Improve error messages
- [ ] Add more comprehensive error handling
- [ ] Review and refactor large functions

## Compatibility

- [ ] Test with various .pptx files
- [ ] Ensure compatibility with Office 2007+
- [ ] Test with LibreOffice
- [ ] Test with Google Slides

## Future Enhancements

- [ ] Animation support
- [ ] Master slide support
- [ ] Notes page support
- [ ] Handout master support
- [ ] Custom XML support
- [ ] VBA macro support
- [ ] Embedded fonts support
- [ ] SmartArt support
- [ ] 3D model support
