# Phase 2 Migration Plan: Achieving python-pptx Parity

## Overview

Based on deep analysis of python-pptx reference file, this document outlines the detailed migration plan to achieve 100% parity with python-pptx behavior and output.

## Current State vs Target State

### Current Implementation (Phase 1 - COMPLETE ✅)
- ✅ Core OPC package handling
- ✅ Basic presentation structure
- ✅ Minimal PPTX generation (5.5 KB, 12 files)
- ✅ 208 tests passing
- ✅ Valid but empty presentations

### Target Implementation (Phase 2 - IN PROGRESS)
- ⏳ Complete slide master with layouts
- ⏳ 11 predefined slide layouts
- ⏳ Proper slide creation and management
- ⏳ Full relationship system
- ⏳ Dynamic content type handling
- ⏳ Full-featured PPTX (30+ KB, 42 files)
- ⏳ 300+ tests passing
- ⏳ Feature-complete presentations with slides

## Key Differences: Our Implementation vs python-pptx

### File Structure Comparison

| Component | Our Files | python-pptx | Gap |
|-----------|-----------|-------------|-----|
| Core files | 12 | 8 | ✅ Complete |
| Slide master | 0 | 2 | ❌ Missing |
| Slide layouts | 0 | 22 | ❌ Missing |
| Slides | 0 | 6 | ❌ Missing |
| **Total** | **12** | **42** | **30 files** |

### Relationship Structure

**Our Current Relationships (6 total)**:
```
rId1 → officeDocument (ppt/presentation.xml)
rId2 → thumbnail (docProps/thumbnail.jpeg)
rId3 → core-properties (docProps/core.xml)
rId4 → extended-properties (docProps/app.xml)
```

**python-pptx Relationships (9 total in presentation.xml.rels)**:
```
rId1 → slideMaster (ppt/slideMasters/slideMaster1.xml)
rId2 → printerSettings (ppt/printerSettings/printerSettings1.bin)
rId3 → presProps (ppt/presProps.xml)
rId4 → viewProps (ppt/viewProps.xml)
rId5 → theme (ppt/theme/theme1.xml)
rId6 → tableStyles (ppt/tableStyles.xml)
rId7 → slide (ppt/slides/slide1.xml)
rId8 → slide (ppt/slides/slide2.xml)
rId9 → slide (ppt/slides/slide3.xml)
```

### XML Structure Differences

**presentation.xml Attributes**:
- ✅ saveSubsetFonts="1" - Already implemented
- ✅ autoCompressPictures="0" - Already implemented
- ✅ defaultTextStyle - Already implemented with 9 levels

**presentation.xml Elements Order** (python-pptx):
1. sldMasterIdLst - ✅ Have reference
2. sldIdLst - ❌ Empty (need to populate with slides)
3. sldSz - ✅ Have it
4. notesSz - ✅ Have it
5. defaultTextStyle - ✅ Have it

**Slide Master Structure**:
```xml
<p:sldMaster>
  <p:cSld>
    <p:bg/>
    <p:spTree>
      <!-- Master shapes -->
    </p:spTree>
  </p:cSld>
  <p:clrMap/>
  <p:sldLayoutIdLst>
    <!-- 11 layout references -->
  </p:sldLayoutIdLst>
  <p:txStyles/>
</p:sldMaster>
```

**Slide Layout Structure**:
```xml
<p:sldLayout>
  <p:cSld>
    <p:bg/>
    <p:spTree>
      <!-- Layout placeholders -->
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr/>
</p:sldLayout>
```

**Slide Structure**:
```xml
<p:sld>
  <p:cSld>
    <p:bg/> <!-- Optional background -->
    <p:spTree>
      <!-- Slide content shapes -->
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr/>
</p:sld>
```

## Implementation Strategy

### Phase 2.1: Slide Master & Layouts (HIGH PRIORITY)

#### Step 1: Create SlideMaster Struct
```rust
// src/slide/master.rs
pub struct SlideMaster {
    part: Option<SlideMasterPart>,
    layouts: Vec<SlideLayout>,
}

impl SlideMaster {
    pub fn new() -> Self { ... }
    pub fn add_layout(&mut self, layout: SlideLayout) { ... }
    pub fn to_xml(&self) -> String { ... }
}
```

#### Step 2: Create SlideLayout Struct
```rust
// src/slide/layout.rs
pub struct SlideLayout {
    part: Option<SlideLayoutPart>,
    layout_type: LayoutType,
}

pub enum LayoutType {
    Blank,
    Title,
    TitleAndContent,
    TitleOnly,
    Centered,
    TitleAndTwoContent,
    // ... 6 more types
}
```

#### Step 3: Generate 11 Predefined Layouts
- Blank
- Title Slide
- Title and Content
- Title Only
- Centered Title
- Title and Two Content
- Comparison
- Title, Content and Caption
- Picture with Caption
- Blank (duplicate)
- Title and Vertical Content

#### Step 4: XML Generation
- Generate slideMaster1.xml with sldLayoutIdLst
- Generate slideLayout1.xml through slideLayout11.xml
- Create relationship files for each

#### Step 5: Update save() Function
- Add master to package
- Add all 11 layouts to package
- Update Content_Types.xml

**Expected Output**:
- 24 new files (1 master + 11 layouts + 12 relationship files)
- 15+ new tests
- slideMaster1.xml: ~12KB
- Each layout: ~2-5KB

### Phase 2.2: Slide Management (HIGH PRIORITY)

#### Step 1: Update Slides::add_slide()
- Create actual slide file (slide1.xml, slide2.xml, etc.)
- Generate slide relationship file
- Add to package
- Update presentation.xml sldIdLst
- Assign unique slide ID

#### Step 2: Implement Slide ID Management
```rust
pub struct SlideId {
    id: u32,
    rel_id: String,
}
```

#### Step 3: Update presentation.xml
- Populate sldIdLst with slide IDs
- Maintain proper ID sequence

**Expected Output**:
- Slide files created on demand
- Proper ID management
- 10+ new tests

### Phase 2.3: Relationship Management (HIGH PRIORITY)

#### Step 1: Update save() Relationships
- Ensure proper rId numbering
- Add all 9 required relationships
- Add slide relationships (rId7+)

#### Step 2: Implement Relationship Ordering
```
rId1 → slideMaster
rId2 → printerSettings
rId3 → presProps
rId4 → viewProps
rId5 → theme
rId6 → tableStyles
rId7+ → slides (in order)
```

**Expected Output**:
- Correct relationship structure
- 8+ new tests

### Phase 2.4: Content Types & Binary (MEDIUM PRIORITY)

#### Step 1: Dynamic Content Type Management
- Add Default entries for .bin, .jpeg, .rels, .xml
- Add Override entries for all parts
- Update when slides are added

#### Step 2: Printer Settings Binary
- Generate actual binary data (not empty)
- Proper structure matching python-pptx

#### Step 3: Thumbnail Generation
- Generate proper JPEG thumbnail
- Update when presentation changes

**Expected Output**:
- Proper [Content_Types].xml
- Real printer settings binary
- Real thumbnail image
- 10+ new tests

## Testing Strategy

### Unit Tests (Per Component)
- SlideMaster creation and XML generation: 5 tests
- SlideLayout creation and XML generation: 5 tests
- Slide creation and ID management: 5 tests
- Relationship management: 5 tests
- Content types: 4 tests
- Binary file handling: 3 tests

### Integration Tests
- Full presentation with 3 slides: 1 test
- Presentation save/load roundtrip: 1 test
- Comparison with python-pptx output: 1 test
- File structure validation: 1 test

### Total: 30+ new tests

## Success Metrics

### Quantitative
- ✅ 300+ tests passing (from 226)
- ✅ 42 files in generated PPTX (from 12)
- ✅ 30+ KB file size (from 5.5 KB)
- ✅ 100% relationship parity
- ✅ 100% content type parity

### Qualitative
- ✅ Generated PPTX matches python-pptx structure exactly
- ✅ All slides properly created and linked
- ✅ Proper slide master and layouts
- ✅ Correct relationship management
- ✅ Can open in PowerPoint with slides visible

## Timeline

### Week 1: Foundation
- Implement SlideMaster and SlideLayout structs
- Generate 11 predefined layouts
- Create XML generation
- Tests: 15+

### Week 2: Slide Management & Relationships
- Update Slides::add_slide()
- Implement slide ID management
- Complete relationship system
- Tests: 18+

### Week 3: Content Types & Polish
- Dynamic content type management
- Printer settings and thumbnail
- Final testing and validation
- Tests: 10+

## Risk Mitigation

### Risk 1: XML Structure Complexity
- **Mitigation**: Use python-pptx reference as template
- **Backup**: Generate XML from known good examples

### Risk 2: Relationship Management
- **Mitigation**: Implement rId tracking system
- **Backup**: Manual rId assignment

### Risk 3: File Size Increase
- **Mitigation**: Use compression in ZIP
- **Backup**: Optimize XML generation

## Dependencies

- No new external dependencies required
- Leverage existing OPC and XML infrastructure
- Use existing Part trait system

## Rollback Plan

- Keep Phase 1 implementation stable
- Feature flags for Phase 2 components
- Gradual integration testing

## Next Steps

1. ✅ Analyze python-pptx reference (DONE)
2. ✅ Create detailed migration plan (DONE)
3. ⏳ Implement SlideMaster struct
4. ⏳ Implement SlideLayout struct
5. ⏳ Generate 11 predefined layouts
6. ⏳ Update Slides::add_slide()
7. ⏳ Complete relationship system
8. ⏳ Dynamic content type management
9. ⏳ Comprehensive testing
10. ⏳ Validation against python-pptx

## Conclusion

This migration plan provides a clear path to achieving 100% parity with python-pptx. By following the phased approach, we can systematically implement all required components while maintaining code quality and test coverage.

The key insight from analyzing python-pptx is that the structure is well-defined and predictable. By implementing the slide master, layouts, and proper relationship management, we can generate presentations that are indistinguishable from python-pptx output.
