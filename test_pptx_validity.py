#!/usr/bin/env python3
"""
Test script to verify PPTX files are valid and can be opened.
"""

import zipfile
import xml.etree.ElementTree as ET
import sys
from pathlib import Path

def validate_pptx(filepath):
    """Validate a PPTX file."""
    print(f"Validating: {filepath}")
    
    try:
        # Check if it's a valid ZIP
        with zipfile.ZipFile(filepath, 'r') as zip_ref:
            print("✓ Valid ZIP archive")
            
            # Check for essential files
            namelist = zip_ref.namelist()
            print(f"✓ Contains {len(namelist)} files")
            
            required_files = ['[Content_Types].xml', '_rels/.rels', 'ppt/presentation.xml']
            for req_file in required_files:
                if req_file in namelist:
                    print(f"✓ Found {req_file}")
                else:
                    print(f"✗ Missing {req_file}")
                    return False
            
            # Validate XML files
            for xml_file in ['[Content_Types].xml', '_rels/.rels', 'ppt/presentation.xml']:
                try:
                    with zip_ref.open(xml_file) as f:
                        content = f.read()
                        ET.fromstring(content)
                        print(f"✓ Valid XML: {xml_file}")
                except Exception as e:
                    print(f"✗ Invalid XML in {xml_file}: {e}")
                    return False
            
            # Check core properties if present
            if 'docProps/core.xml' in namelist:
                try:
                    with zip_ref.open('docProps/core.xml') as f:
                        content = f.read()
                        ET.fromstring(content)
                        print(f"✓ Valid XML: docProps/core.xml")
                except Exception as e:
                    print(f"✗ Invalid XML in docProps/core.xml: {e}")
                    return False
            
        print(f"✓ File is valid and can be opened!")
        return True
        
    except zipfile.BadZipFile:
        print("✗ Not a valid ZIP file")
        return False
    except Exception as e:
        print(f"✗ Error: {e}")
        return False

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python test_pptx_validity.py <pptx_file>")
        sys.exit(1)
    
    filepath = sys.argv[1]
    if not Path(filepath).exists():
        print(f"✗ File not found: {filepath}")
        sys.exit(1)
    
    success = validate_pptx(filepath)
    sys.exit(0 if success else 1)
