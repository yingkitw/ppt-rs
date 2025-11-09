#!/usr/bin/env python3
import sys
from pptx import Presentation

def test_open(filename):
    try:
        prs = Presentation(filename)
        print(f"✓ {filename}: Successfully opened")
        print(f"  - Slides: {len(prs.slides)}")
        print(f"  - Slide masters: {len(prs.slide_masters)}")
        return True
    except Exception as e:
        print(f"✗ {filename}: Failed to open")
        print(f"  Error: {e}")
        return False

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 test_open.py <pptx_file>")
        sys.exit(1)
    
    success = test_open(sys.argv[1])
    sys.exit(0 if success else 1)
