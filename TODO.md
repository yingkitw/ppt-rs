# TODO - PPTX Rust Translation

## High Priority

### 1. ZIP File Handling
- [ ] Implement ZIP reading in `opc/package.rs`
- [ ] Implement ZIP writing in `opc/package.rs`
- [ ] Handle ZIP relationships
- [ ] Extract content types

### 2. XML Parsing & Serialization
- [ ] Implement XML parsing in `oxml/xmlchemy.rs`
- [ ] Implement XML serialization
- [ ] Handle namespaces properly
- [ ] Create element factory

### 3. Parts Implementation
- [ ] `parts/presentation.rs` - PresentationPart
- [ ] `parts/slide.rs` - SlidePart
- [ ] `parts/image.rs` - ImagePart
- [ ] `parts/chart.rs` - ChartPart
- [ ] `parts/coreprops.rs` - CorePropertiesPart
- [ ] Implement PartFactory
- [ ] Implement relationships

## Medium Priority

### 4. Shape Implementation
- [ ] `shapes/base.rs` - BaseShape
- [ ] `shapes/autoshape.rs` - AutoShape
- [ ] `shapes/picture.rs` - Picture
- [ ] `shapes/placeholder.rs` - Placeholder
- [ ] `shapes/group.rs` - GroupShape
- [ ] `shapes/shapetree.rs` - ShapeTree
- [ ] Implement shape factory

### 5. Text Implementation
- [ ] `text/text.rs` - TextFrame, Paragraph, Run
- [ ] `text/fonts.rs` - Font handling
- [ ] `text/layout.rs` - Text layout
- [ ] Implement text formatting
- [ ] Implement paragraph formatting

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
- [ ] Create migration guide from python-pptx
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
