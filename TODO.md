# TODO

## Migration Progress

- [x] Scaffold Rust project structure
- [x] Migrate OPC (Open Packaging Convention) package handling - PackURI, PackageReader/Writer, Part trait, Relationships, Constants expanded
- [x] Migrate OpenXML processing (oxml) - XML parsing and generation - basic parsing/writing implemented
- [x] Migrate Parts module - All parts (PresentationPart, SlidePart, SlideLayoutPart, SlideMasterPart, NotesMasterPart, NotesSlidePart, ImagePart, ChartPart, CorePropertiesPart, MediaPart) implemented with Part trait
- [x] Migrate Presentation and Slide core classes - Presentation and Slide classes implemented with basic functionality
- [x] Migrate Shapes module - BaseShape trait enhanced, AutoShape, Picture, Connector, GraphicFrame, GroupShape implemented
- [x] **Expand AutoShapeType enum** - Added 100+ shape types (basic shapes, arrows, flowchart shapes, callouts, action buttons)
- [x] Migrate Text module - TextFrame, Paragraph, Font implemented with basic functionality
- [x] Migrate Table module - Table, TableRow, TableColumn, TableCell implemented
- [x] Migrate Chart module - Chart, ChartTitle, ChartSeries, ChartLegend implemented
- [x] **Expand ChartType enum** - Added 100+ chart types (Area, Bar, Column, Line, Pie, Scatter, Bubble, Radar, Stock, Surface, Cone, Cylinder, Pyramid, 3D variants)
- [x] Migrate DML (DrawingML) - ColorFormat, FillFormat, LineFormat implemented
- [x] **Expand DashStyle enum** - Added 13+ dash styles (Solid, Dash, Dot, LongDash, SystemDash, etc.)
- [x] Migrate Enums module - ShapeType, PlaceholderType, TextAlign, ChartType, ColorType, FillType expanded
- [x] Create main API entry point - Presentation function and helper functions implemented
- [x] **Implement Hyperlink support** - Hyperlink struct with address and screen_tip, integrated into AutoShape and Picture
- [x] **Implement Shape XML parsing** - parse_shapes_from_xml, shape_to_xml, next_shape_id functions
- [x] **Implement Slide shape operations** - Slide::shapes() and Slide::add_shape() with XML parsing/generation
- [x] Migrate tests and ensure cargo test passes - Comprehensive tests added: **144 tests passing**

## Current Task

✅ **CORE MIGRATION COMPLETE!** All major modules implemented and tests passing. The Rust codebase successfully mirrors the Python python-pptx library structure.

## Recent Enhancements

- ✅ **Expanded Enums:**
  - ChartType: 100+ chart types (Area, Bar, Column, Line, Pie, Scatter, Bubble, Radar, Stock, Surface, Cone, Cylinder, Pyramid, 3D variants)
  - AutoShapeType: 100+ shape types (basic shapes, arrows, flowchart shapes, callouts, action buttons)
  - DashStyle: 13+ dash styles (Solid, Dash, Dot, LongDash, SystemDash, etc.)

- ✅ **Hyperlink Support:**
  - Hyperlink struct with address and screen_tip
  - Integrated into AutoShape and Picture shapes
  - XML generation and parsing support

- ✅ **Shape XML Operations:**
  - parse_shapes_from_xml() - Parse shapes from slide XML spTree
  - shape_to_xml() - Generate XML for shapes
  - next_shape_id() - Find next available shape ID
  - Support for AutoShape, Picture, and Connector parsing

- ✅ **Slide Shape Management:**
  - Slide::shapes() - Parse and return shapes from slide XML
  - Slide::add_shape() - Add shapes to slide XML
  - SlidePart::update_xml() - Update slide part XML content

- ✅ **Test Coverage:**
  - Increased from 84 to **144 tests** (+71%)
  - Added tests for hyperlinks, shape XML operations, expanded enums, LineFormat, Slide operations, image handling, and PPTX save validation

- ✅ **Recent Bug Fixes and Enhancements (Nov 8, 2025):**
  - Fixed regex pattern matching in Slides::len() and PresentationPart::next_slide_partname()
  - Fixed relationship preservation in PresentationPart::add_slide()
  - Fixed relationship preservation in SlidePart::update_xml()
  - Fixed picture shape XML generation with image references
  - Fixed slide blob updates when adding images to slides
  - Added explicit flush() call in PackageWriter to ensure all data is written
  - Added comprehensive test_save_and_validate_pptx() test that validates:
    - PPTX files can be saved and opened as valid ZIP archives
    - [Content_Types].xml is properly generated
    - _rels/.rels contains correct relationships
    - presentation.xml has proper XML structure and namespaces
  - All 144 tests now passing

## Notes

- Code compiles successfully with minimal warnings
- All core parts implement the Part trait
- Presentation and Slide classes have full functionality
- Shapes module with BaseShape, AutoShape, Picture, Connector, GraphicFrame, GroupShape
- **100+ AutoShape types** available
- **Hyperlink support** for shapes
- **Shape XML parsing and generation** working
- Text module with TextFrame, Paragraph, Font
- Table module with Table, TableRow, TableColumn, TableCell
- Chart module with Chart, ChartTitle, ChartSeries, ChartLegend, CategoryAxis, ValueAxis, DateAxis
- **100+ Chart types** available
- DML module with ColorFormat, FillFormat, LineFormat
- **13+ Line dash styles** available
- Enums expanded with ShapeType, PlaceholderType, TextAlign, ChartType, ColorType, FillType
- API entry point functions available
- **144 tests passing** covering:
  - PackURI (9 tests)
  - Relationships (6 tests)
  - TextFrame (5 tests)
  - Paragraph (5 tests)
  - Font (3 tests)
  - Shapes (13+ tests): BaseShape, AutoShape, Picture, Connector, GraphicFrame, GroupShape, Hyperlink integration
  - Shape XML (12 tests): Parsing, generation, transform parsing
  - Table (5 tests)
  - Chart (6 tests)
  - Chart Axes (5 tests)
  - DML (13+ tests): RGBColor, ColorFormat, FillFormat, LineFormat with all dash styles
  - Enums (7 tests): ChartType, AutoShapeType equality and operations
  - Slide (5 tests): Basic operations, masters, layouts
  - Hyperlink (10 tests): Creation, XML generation/parsing, escape/unescape
  - Presentation (11+ tests): Save, open, slide dimensions, image handling, slide collection
  - OPC Serialization (4 tests): Content types, relationships, package reader, save and validate PPTX

## Pending Features

- [ ] Table styles - Table style management and application
- [ ] Text hyperlinks - Hyperlink support for text runs
- [ ] Gradient fills - Gradient fill support for shapes
- [ ] Pattern fills - Pattern fill support
- [ ] Picture fills - Picture fill support
- [ ] Slide backgrounds - Background formatting
- [ ] Slide transitions - Transition effects
- [ ] Placeholder shapes - Placeholder shape support
- [ ] Advanced chart features - Chart data management, Excel integration
- [ ] Freeform shapes - Freeform shape support
- [ ] Shadow effects - Shadow effect support for shapes
- [ ] OLE objects - OLE object support
