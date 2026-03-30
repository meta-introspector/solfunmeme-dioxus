#!/bin/bash

# Solfunmeme-Dioxus UML Diagram Generator
# This script generates PNG images from all PlantUML files in the current directory

echo "🌐 Generating Solfunmeme-Dioxus UML Diagrams..."
echo "=============================================="

# Check if PlantUML is installed
if ! command -v plantuml &> /dev/null; then
    echo "❌ PlantUML is not installed. Please install it first:"
    echo "   macOS: brew install plantuml"
    echo "   Ubuntu: sudo apt-get install plantuml"
    echo "   Or download from: https://plantuml.com/download"
    exit 1
fi

# Create output directory
mkdir -p generated

# Generate diagrams
echo "📊 Generating C4 Architecture Diagrams..."

echo "  🔍 Context Diagram..."
plantuml -tpng -o generated 01-context-diagram.puml

echo "  📦 Container Diagram..."
plantuml -tpng -o generated 02-container-diagram.puml

echo "  🧩 Component Diagram..."
plantuml -tpng -o generated 03-component-diagram.puml

echo "  🚀 Deployment Diagram..."
plantuml -tpng -o generated 04-deployment-diagram.puml

echo "📋 Generating Additional Diagrams..."

echo "  👥 Use Case Diagram..."
plantuml -tpng -o generated 05-use-case-diagram.puml

echo "  🔄 Data Flow Diagram..."
plantuml -tpng -o generated 06-data-flow-diagram.puml

echo "  ⏱️ Sequence Diagram..."
plantuml -tpng -o generated 07-sequence-diagram.puml

echo "  ⛓️ L2 Sidechain Architecture..."
plantuml -tpng -o generated 08-l2-sidechain-architecture.puml

echo "  🔗 Cross-References..."
plantuml -tpng -o generated 09-cross-references.puml

echo ""
echo "✅ All diagrams generated successfully!"
echo "📁 Generated files are in the 'generated' directory:"
echo ""

# List generated files
ls -la generated/*.png 2>/dev/null || echo "No PNG files found in generated directory"

echo ""
echo "🎯 Generated Diagrams:"
echo "====================="
echo "1. 01-context-diagram.png      - System context and external dependencies"
echo "2. 02-container-diagram.png    - Main containers and technology choices"
echo "3. 03-component-diagram.png    - Detailed component interactions"
echo "4. 04-deployment-diagram.png   - Infrastructure and deployment"
echo "5. 05-use-case-diagram.png     - User interactions and system functionality"
echo "6. 06-data-flow-diagram.png    - Data transformation through 8 factorial steps"
echo "7. 07-sequence-diagram.png     - Component interactions during pipeline execution"
echo "8. 08-l2-sidechain-architecture.png - L2 sidechain storage and mainnet rollup"
echo "9. 09-cross-references.png - Documentation relationships and navigation"
echo ""
echo "📖 For more information, see README.md" 