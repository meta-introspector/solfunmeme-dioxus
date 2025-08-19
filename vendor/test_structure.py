#!/usr/bin/env python3
"""
Test script to demonstrate the hierarchical vendor structure
without actually moving any files.
"""

from pathlib import Path
import sys

# Import the migration functions
sys.path.append('.')
from migrate_to_hierarchy import HIERARCHY, SPECIAL_MAPPINGS, find_dependency_location

def print_structure():
    """Print the complete hierarchical structure."""
    print("Vendor Hierarchical Structure (7-5-3-2)")
    print("=" * 50)
    
    for i, (layer, subcategories) in enumerate(HIERARCHY.items(), 1):
        print(f"\n{i}. {layer.upper()} LAYER")
        print("-" * 30)
        
        for j, (subcategory, deps) in enumerate(subcategories.items(), 1):
            print(f"  {j}. {subcategory}/")
            print(f"     ├── core/")
            print(f"     │   ├── stable/")
            print(f"     │   └── experimental/")
            print(f"     ├── bindings/")
            print(f"     │   ├── stable/")
            print(f"     │   └── experimental/")
            print(f"     └── extensions/")
            print(f"         ├── stable/")
            print(f"         └── experimental/")
            
            if deps:
                print(f"     Dependencies: {', '.join(deps[:3])}{'...' if len(deps) > 3 else ''}")

def test_dependency_mapping():
    """Test mapping of some dependencies."""
    print("\n\nDependency Mapping Examples")
    print("=" * 40)
    
    test_deps = [
        "serde", "hyper", "tokio", "dioxus", "rustls", 
        "nalgebra", "tantivy", "candle", "solana-sdk",
        "unknown-dependency"
    ]
    
    for dep in test_deps:
        location = find_dependency_location(dep)
        status = "✓" if location else "✗"
        print(f"{status} {dep:20} -> {location or 'UNMAPPED'}")

def count_dependencies():
    """Count dependencies in each category."""
    print("\n\nDependency Counts by Layer")
    print("=" * 30)
    
    total_deps = 0
    for layer, subcategories in HIERARCHY.items():
        layer_count = sum(len(deps) for deps in subcategories.values())
        total_deps += layer_count
        print(f"{layer:12}: {layer_count:3} dependencies")
    
    special_count = len(SPECIAL_MAPPINGS)
    total_deps += special_count
    print(f"{'special':12}: {special_count:3} dependencies")
    print(f"{'TOTAL':12}: {total_deps:3} dependencies")

def show_osi_analogy():
    """Show the OSI layer analogy."""
    print("\n\nOSI Layer Analogy")
    print("=" * 20)
    
    osi_mapping = {
        "crypto": "Layer 1: Physical (Security Foundation)",
        "system": "Layer 2: Data Link (System Integration)", 
        "network": "Layer 3: Network (Communication)",
        "data": "Layer 4: Transport (Data Management)",
        "compute": "Layer 5: Session (Computation)",
        "ui": "Layer 6: Presentation (User Interface)",
        "ai": "Layer 7: Application (Intelligence)"
    }
    
    for layer, description in osi_mapping.items():
        print(f"{layer:10}: {description}")

if __name__ == "__main__":
    print_structure()
    test_dependency_mapping()
    count_dependencies()
    show_osi_analogy()
    
    print("\n\nStructure Summary")
    print("=" * 20)
    print("7 Layers (OSI-like)")
    print("├── 5 Subcategories per layer")
    print("│   ├── 3 Components per subcategory")
    print("│   │   ├── 2 Levels per component")
    print("│   │   │   ├── stable/")
    print("│   │   │   └── experimental/")
    print("│   │   ├── core/")
    print("│   │   ├── bindings/")
    print("│   │   └── extensions/")
    print("│   ├── primitives/")
    print("│   ├── protocols/")
    print("│   ├── keys/")
    print("│   ├── hashes/")
    print("│   └── utils/")
    print("├── crypto/")
    print("├── system/")
    print("├── network/")
    print("├── data/")
    print("├── compute/")
    print("├── ui/")
    print("└── ai/") 