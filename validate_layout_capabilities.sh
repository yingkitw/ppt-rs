#!/bin/bash
# Validation script for layout, template, and footer capabilities (TODO 29–32).

set -euo pipefail

echo "🧪 ppt-rs Layout & Template Validation"
echo "======================================="
echo ""

echo "📋 Running layout packaging tests..."
cargo test --test layouts_packaging_test --quiet
echo "✅ Layout packaging tests passed"
echo ""

echo "📋 Running PowerPoint compatibility tests..."
cargo test --test powerpoint_compat_test --quiet
echo "✅ PowerPoint compatibility tests passed"
echo ""

echo "📋 Running layout integration checks..."
cargo test --test integration_tests test_create_presentation_with_all_layouts --quiet
cargo test --test output_quality_test test_all_layouts_generate_valid_pptx --quiet
echo "✅ Layout integration tests passed"
echo ""

echo "📄 Checking example files..."
for f in examples/layout_template_features.rs examples/layout_demo.rs; do
    if [ -f "$f" ]; then
        echo "✅ $f"
    else
        echo "❌ Missing $f"
        exit 1
    fi
done
echo ""

echo "🏗️  Building layout template example..."
cargo run --example layout_template_features --quiet
echo "✅ Example ran successfully"
echo ""

for out in \
    examples/output/layout_template_features.pptx \
    examples/output/from_template.pptx \
    examples/output/layout_only.pptx; do
    if [ -f "$out" ]; then
        echo "✅ $out"
    else
        echo "❌ Missing output $out"
        exit 1
    fi
done
echo ""

echo "🎉 All layout/template/footer validations passed."
