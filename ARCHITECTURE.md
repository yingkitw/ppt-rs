# Architecture

## Overview

ppt-rs is structured following the OpenXML standard for PowerPoint files. A .pptx file is essentially a ZIP archive containing XML files and media resources.

## Module Structure

```mermaid
graph LR
    A[ppt_rs] --> B[opc]
    A --> C[oxml]
    A --> D[parts]
    A --> E[shapes]
    A --> F[text]
    A --> G[chart]
    A --> H[dml]
    A --> I[enums]
    B --> J[Package]
    B --> K[Part]
    C --> L[XML Processing]
    D --> M[PresentationPart]
    D --> N[SlidePart]
    D --> O[ImagePart]
```

## Core Components

### OPC (Open Packaging Convention)
- Handles ZIP archive structure
- Manages parts and relationships
- Serialization/deserialization

### OpenXML (oxml)
- XML parsing and generation
- Type-safe XML element handling
- Schema validation

### Parts
- PresentationPart: Main presentation document
- SlidePart: Individual slides
- ImagePart: Image resources
- ChartPart: Chart data
- MediaPart: Video/audio resources

### Shapes
- Base shape functionality
- AutoShapes, Pictures, Connectors
- Group shapes

### Text
- Text formatting
- Paragraph and run handling
- Font management

### Chart
- Chart creation and modification
- Data series handling
- Chart formatting

### DML (DrawingML)
- Colors, fills, lines
- Effects and formatting

