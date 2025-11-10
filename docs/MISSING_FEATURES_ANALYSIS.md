# Missing Features Analysis - Critical Issues

## Overview

Investigation revealed **6 critical missing features** that are essential for full python-pptx compatibility. These features may cause issues when generating presentations that use common patterns.

---

## Critical Missing Features

### 1. **Placeholders** (CRITICAL)
**Status**: Partially implemented  
**Impact**: HIGH - Blocks common use cases

#### What's Missing
- Direct access to `shapes.title` property
- Access to `placeholders[i]` collection
- Placeholder type detection and filtering
- Placeholder shape management

#### python-pptx Usage
```python
slide = prs.slides.add_slide(prs.slide_layouts[1])
title = slide.shapes.title  # ← Missing
subtitle = slide.placeholders[1]  # ← Missing
title.text = "Title"
subtitle.text = "Subtitle"
```

#### Why It's Critical
- Most presentations use placeholder shapes for content
- Placeholder access is the primary way to add text to slides
- Without this, users must manually create shapes instead

#### Implementation Needed
- [ ] `Slide::shapes_title()` method
- [ ] `Slide::placeholders()` collection
- [ ] `PlaceholderShape` struct
- [ ] Placeholder type detection
- [ ] Placeholder-to-shape mapping

---

### 2. **Notes Slides** (CRITICAL)
**Status**: Not implemented  
**Impact**: HIGH - Blocks speaker notes

#### What's Missing
- Notes slide creation and access
- Notes text frame management
- Notes content persistence in XML

#### python-pptx Usage
```python
slide = prs.slides.add_slide(prs.slide_layouts[0])
notes_slide = slide.notes_slide  # ← Missing
text_frame = notes_slide.notes_text_frame  # ← Missing
text_frame.text = "Speaker notes here"
```

#### Why It's Critical
- Speaker notes are essential for presentations
- Many presentations rely on notes for speaker guidance
- Notes must be preserved when saving/loading

#### Implementation Needed
- [ ] `NotesSlide` struct
- [ ] `Slide::notes_slide()` method
- [ ] `NotesSlide::notes_text_frame()` method
- [ ] Notes XML generation and parsing
- [ ] Notes slide relationships

---

### 3. **Core Properties** (CRITICAL)
**Status**: Partially implemented  
**Impact**: HIGH - Blocks metadata

#### What's Missing
- Proper core properties access and modification
- Metadata persistence in docProps/core.xml
- Timestamp management (created, modified)

#### python-pptx Usage
```python
prs.core_properties.title = "My Presentation"
prs.core_properties.author = "John Doe"
prs.core_properties.subject = "Test"
prs.core_properties.keywords = "test, presentation"
prs.core_properties.comments = "This is a test"
```

#### Why It's Critical
- Metadata is important for document management
- File properties are visible in Windows/Mac file explorers
- Required for document tracking and organization

#### Implementation Needed
- [ ] `CoreProperties` struct with all properties
- [ ] Getters and setters for all metadata fields
- [ ] Timestamp auto-update on save
- [ ] XML serialization to docProps/core.xml
- [ ] Proper XML parsing for loading

---

### 4. **Slide Names** (IMPORTANT)
**Status**: Partially implemented  
**Impact**: MEDIUM - Blocks custom naming

#### What's Missing
- Proper slide name property with persistence
- Name storage in slide XML
- Name retrieval and modification

#### python-pptx Usage
```python
slide = prs.slides.add_slide(prs.slide_layouts[0])
slide.name = "Custom Slide Name"  # ← Not persisted
print(slide.name)  # ← May not work correctly
```

#### Why It's Important
- Custom slide names help organize presentations
- Names appear in slide sorter view
- Names are useful for navigation

#### Implementation Needed
- [ ] `Slide::name()` getter
- [ ] `Slide::set_name()` setter
- [ ] Name storage in slide XML (p:cSld/@name)
- [ ] Name persistence on save

---

### 5. **Slide Layouts Collection** (IMPORTANT)
**Status**: Partially implemented  
**Impact**: MEDIUM - Blocks layout selection

#### What's Missing
- Proper `slide_layouts` collection access
- Layout enumeration
- Layout properties and metadata
- Layout-to-slide mapping

#### python-pptx Usage
```python
for layout in prs.slide_layouts:
    print(layout.name)  # ← May not work

slide = prs.slides.add_slide(prs.slide_layouts[1])  # ← Works but limited
```

#### Why It's Important
- Users need to select appropriate layouts
- Layout names help identify correct layout
- Layout properties are useful for validation

#### Implementation Needed
- [ ] `SlideLayouts` collection struct
- [ ] `SlideLayout::name()` property
- [ ] `SlideLayout::placeholders()` collection
- [ ] Proper layout enumeration
- [ ] Layout metadata access

---

### 6. **Slide Master** (IMPORTANT)
**Status**: Partially implemented  
**Impact**: MEDIUM - Blocks master access

#### What's Missing
- Proper `slide_master` property access
- `slide_masters` collection
- Master properties and metadata
- Master-to-slide relationships

#### python-pptx Usage
```python
master = prs.slide_master  # ← May not work
masters = prs.slide_masters  # ← May not work
print(master.name)  # ← May not work
```

#### Why It's Important
- Master slides control overall presentation design
- Users may need to access/modify master properties
- Master relationships are critical for structure

#### Implementation Needed
- [ ] `Slide::slide_master()` method
- [ ] `Presentation::slide_masters()` collection
- [ ] `SlideMaster` properties and metadata
- [ ] Master-to-layout relationships
- [ ] Proper master XML handling

---

## Impact Analysis

### Blocking Issues
These features **MUST** be implemented for basic compatibility:
1. **Placeholders** - Without this, users can't add text to slides easily
2. **Core properties** - Without this, metadata is lost
3. **Notes slides** - Without this, speaker notes are lost

### Important Issues
These features **SHOULD** be implemented for good compatibility:
1. **Slide names** - Improves usability
2. **Slide layouts** - Improves layout selection
3. **Slide master** - Improves design control

---

## Implementation Priority

### Phase 7 (Critical)
1. **Placeholders** - Highest priority
2. **Core properties** - High priority
3. **Notes slides** - High priority

### Phase 8 (Important)
1. **Slide names** - Medium priority
2. **Slide layouts** - Medium priority
3. **Slide master** - Medium priority

---

## Testing Strategy

### For Each Feature
1. Create comprehensive test cases
2. Compare with python-pptx output
3. Verify XML structure
4. Test round-trip (save/load)
5. Validate persistence

### Test Files
- `test_placeholders.rs` - Placeholder access and manipulation
- `test_notes_slides.rs` - Notes slide creation and content
- `test_core_properties.rs` - Metadata management
- `test_slide_names.rs` - Custom naming
- `test_slide_layouts.rs` - Layout enumeration
- `test_slide_master.rs` - Master access

---

## Estimated Effort

| Feature | Complexity | Effort | Priority |
|---------|-----------|--------|----------|
| Placeholders | High | 3-4 days | CRITICAL |
| Core properties | Medium | 2-3 days | CRITICAL |
| Notes slides | High | 3-4 days | CRITICAL |
| Slide names | Low | 1 day | IMPORTANT |
| Slide layouts | Medium | 2 days | IMPORTANT |
| Slide master | Medium | 2 days | IMPORTANT |

**Total Estimated Effort**: 13-17 days

---

## Conclusion

These 6 features are **essential for full python-pptx compatibility**. Without them, users will encounter issues when:
- Adding text to slides (placeholders)
- Adding speaker notes (notes slides)
- Managing document metadata (core properties)
- Organizing slides (slide names)
- Selecting layouts (slide layouts)
- Accessing design elements (slide master)

**Recommendation**: Implement these features in Phase 7-8 to achieve production-ready parity with python-pptx.

