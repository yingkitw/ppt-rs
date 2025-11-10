# Relationship Order Fix

## Problem
PowerPoint was showing "found a problem with content" error because relationships were in the wrong order.

## Root Cause
1. **HashMap doesn't preserve order** - `Relationships` was using `HashMap<String, Relationship>`, which doesn't maintain insertion order
2. **Wrong relationship order** - We had: rId1=master, rId2-5=slides, rId6-9=properties
3. **Python-pptx uses different order** - Python-pptx: rId1=master, rId2=printerSettings, rId3-6=properties, rId7+=slides

## Solution Applied

### 1. Fixed HashMap Issue
**File**: `src/opc/relationships.rs`

Changed from `HashMap` to `IndexMap` to preserve insertion order:
```rust
// Before
use std::collections::HashMap;
pub struct Relationships {
    relationships: HashMap<String, Relationship>,
    ...
}

// After
use indexmap::IndexMap;
pub struct Relationships {
    relationships: IndexMap<String, Relationship>,
    ...
}
```

**Added dependency** in `Cargo.toml`:
```toml
indexmap = "2.0"
```

### 2. Fixed Relationship Order
**File**: `src/presentation/save.rs`

Changed to match python-pptx order (without printerSettings):
```rust
// New order:
// rId1: slideMaster
// rId2: presProps
// rId3: viewProps
// rId4: theme
// rId5: tableStyles
// rId6+: slides
```

**File**: `src/presentation/presentation.rs`

Updated slide rId generation:
```rust
// Before: rId2 onwards
let r_id = format!("rId{}", 2 + slide_count);

// After: rId6 onwards
let r_id = format!("rId{}", 6 + slide_count);
```

## Comparison

### Python-pptx Order
```
rId1: slideMaster
rId2: printerSettings (optional, we skip this)
rId3: presProps
rId4: viewProps
rId5: theme
rId6: tableStyles
rId7+: slides
```

### Our Order (Current)
```
rId1: slideMaster
rId2: presProps
rId3: viewProps
rId4: theme
rId5: tableStyles
rId6+: slides
```

### PowerPoint Repaired Order
```
(Random order - PowerPoint reorganizes during repair)
```

## Key Differences

| Aspect | Python-pptx | Our Implementation | Status |
|--------|-------------|-------------------|--------|
| Order preservation | ✅ | ✅ (IndexMap) | Fixed |
| Master first | ✅ | ✅ | Match |
| Properties before slides | ✅ | ✅ | Match |
| printerSettings | ✅ | ❌ (optional) | Acceptable |
| Slide rIds | rId7+ | rId6+ | Different but valid |

## Testing

### Before Fix
- Relationships in random order (HashMap)
- PowerPoint showed repair prompt
- Relationships reorganized during repair

### After Fix
- Relationships in consistent order (IndexMap)
- Properties before slides (matching python-pptx pattern)
- Need to verify: Does PowerPoint still show repair prompt?

## Next Steps

1. **Test with PowerPoint** - Verify if repair prompt is gone
2. **If repair prompt persists** - Consider adding printerSettings (optional part)
3. **If no repair prompt** - Current implementation is acceptable

## Files Modified

1. `src/opc/relationships.rs` - Changed HashMap to IndexMap
2. `src/presentation/save.rs` - Reordered relationships
3. `src/presentation/presentation.rs` - Updated slide rId generation
4. `Cargo.toml` - Added indexmap dependency

## Benefits

✅ **Consistent order** - Relationships always in same order
✅ **Matches python-pptx pattern** - Properties before slides
✅ **Better compatibility** - Closer to python-pptx behavior
✅ **Predictable output** - No random ordering

## Trade-offs

⚠️ **No printerSettings** - We skip this optional part (saves ~1KB)
⚠️ **Different rId numbers** - Slides start at rId6 instead of rId7
✅ **Both are valid** - OPC spec allows both approaches

## Conclusion

The relationship order has been fixed to match python-pptx's pattern more closely. The key fix was using `IndexMap` instead of `HashMap` to preserve insertion order. We now put properties before slides, which matches python-pptx behavior (except for the optional printerSettings part).

**Status**: Awaiting PowerPoint verification
