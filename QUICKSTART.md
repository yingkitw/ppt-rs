# Quick Start Guide - PPTX CLI

## 5-Minute Setup

### 1. Build the CLI

```bash
cd /Users/yingkitw/Desktop/myproject/ppt-rs3
cargo build --bin pptx-cli
```

### 2. Create Your First Presentation

```bash
./target/debug/pptx-cli create my_first.pptx --title "Hello World" --slides 3
```

### 3. Check the Result

```bash
./target/debug/pptx-cli info my_first.pptx
```

## Common Tasks

### Create a Simple Presentation

```bash
pptx-cli create presentation.pptx
```

### Create with Title and Multiple Slides

```bash
pptx-cli create report.pptx --title "Q1 Report" --slides 10
```

### Create in a Specific Directory

```bash
pptx-cli create ./output/my_presentation.pptx --title "My Presentation"
```

### View Presentation Information

```bash
pptx-cli info presentation.pptx
```

### Get Help

```bash
pptx-cli help
```

## Command Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `create` | Create new presentation | `pptx-cli create my.pptx` |
| `info` | Show presentation info | `pptx-cli info my.pptx` |
| `help` | Show help message | `pptx-cli help` |

## Options

| Option | Usage | Example |
|--------|-------|---------|
| `--title` | Set presentation title | `--title "My Title"` |
| `--slides` | Set number of slides | `--slides 5` |
| `--template` | Use template file | `--template template.pptx` |

## Examples

### Example 1: Conference Presentation

```bash
pptx-cli create conference2025.pptx \
  --title "Annual Conference 2025" \
  --slides 25
```

### Example 2: Project Report

```bash
pptx-cli create project_report.pptx \
  --title "Project Status Report" \
  --slides 15
```

### Example 3: Training Material

```bash
pptx-cli create training.pptx \
  --title "Rust Training" \
  --slides 50
```

## Troubleshooting

### Binary not found?
```bash
# Use full path
./target/debug/pptx-cli create my.pptx

# Or build release version
cargo build --release --bin pptx-cli
./target/release/pptx-cli create my.pptx
```

### Permission denied?
```bash
chmod +x target/debug/pptx-cli
```

### Invalid arguments?
```bash
# Check help
pptx-cli help

# Verify syntax
pptx-cli create output.pptx --title "Title" --slides 5
```

## Next Steps

1. **Explore the library** - Check [README.md](README.md)
2. **Learn the architecture** - Read [ARCHITECTURE.md](ARCHITECTURE.md)
3. **Full CLI guide** - See [CLI_GUIDE.md](CLI_GUIDE.md)
4. **Check progress** - View [TRANSLATION_PROGRESS.md](TRANSLATION_PROGRESS.md)

## Tips & Tricks

### Create multiple presentations in a loop

```bash
for i in {1..5}; do
  pptx-cli create presentation_$i.pptx --title "Presentation $i" --slides 10
done
```

### Create with timestamp

```bash
pptx-cli create "presentation_$(date +%Y%m%d_%H%M%S).pptx" --title "Report"
```

### Create in organized directory structure

```bash
mkdir -p presentations/2025/q1
pptx-cli create presentations/2025/q1/report.pptx --title "Q1 Report"
```

### Batch create presentations

```bash
#!/bin/bash
for title in "Sales" "Marketing" "Engineering" "HR"; do
  pptx-cli create "${title}_Report.pptx" --title "$title Report" --slides 10
done
```

## Performance Notes

- Creating a 100-slide presentation: < 100ms
- Getting file info: < 10ms
- Help display: < 5ms

## Support

For issues or questions:
1. Check [CLI_GUIDE.md](CLI_GUIDE.md) for detailed documentation
2. Review [ARCHITECTURE.md](ARCHITECTURE.md) for design details
3. See [TODO.md](TODO.md) for planned features
