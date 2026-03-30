# Solfunmeme-Dioxus UML Diagram Generator (PowerShell)
# This script generates PNG images from all PlantUML files in the current directory

Write-Host "🌐 Generating Solfunmeme-Dioxus UML Diagrams..." -ForegroundColor Green
Write-Host "==============================================" -ForegroundColor Green

# Check if PlantUML is installed
try {
    $null = Get-Command plantuml -ErrorAction Stop
} catch {
    Write-Host "❌ PlantUML is not installed. Please install it first:" -ForegroundColor Red
    Write-Host "   Download from: https://plantuml.com/download" -ForegroundColor Yellow
    Write-Host "   Or install via Chocolatey: choco install plantuml" -ForegroundColor Yellow
    Write-Host "   Or install via Scoop: scoop install plantuml" -ForegroundColor Yellow
    exit 1
}

# Create output directory
if (!(Test-Path "generated")) {
    New-Item -ItemType Directory -Path "generated" | Out-Null
}

# Generate diagrams
Write-Host "📊 Generating C4 Architecture Diagrams..." -ForegroundColor Cyan

Write-Host "  🔍 Context Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 01-context-diagram.puml

Write-Host "  📦 Container Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 02-container-diagram.puml

Write-Host "  🧩 Component Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 03-component-diagram.puml

Write-Host "  🚀 Deployment Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 04-deployment-diagram.puml

Write-Host "📋 Generating Additional Diagrams..." -ForegroundColor Cyan

Write-Host "  👥 Use Case Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 05-use-case-diagram.puml

Write-Host "  🔄 Data Flow Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 06-data-flow-diagram.puml

Write-Host "  ⏱️ Sequence Diagram..." -ForegroundColor Yellow
plantuml -tpng -o generated 07-sequence-diagram.puml

Write-Host "  ⛓️ L2 Sidechain Architecture..." -ForegroundColor Yellow
plantuml -tpng -o generated 08-l2-sidechain-architecture.puml

Write-Host "  🔗 Cross-References..." -ForegroundColor Yellow
plantuml -tpng -o generated 09-cross-references.puml

Write-Host ""
Write-Host "✅ All diagrams generated successfully!" -ForegroundColor Green
Write-Host "📁 Generated files are in the 'generated' directory:" -ForegroundColor Cyan
Write-Host ""

# List generated files
$generatedFiles = Get-ChildItem "generated\*.png" -ErrorAction SilentlyContinue
if ($generatedFiles) {
    $generatedFiles | ForEach-Object {
        Write-Host "  $($_.Name)" -ForegroundColor White
    }
} else {
    Write-Host "No PNG files found in generated directory" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🎯 Generated Diagrams:" -ForegroundColor Cyan
Write-Host "=====================" -ForegroundColor Cyan
Write-Host "1. 01-context-diagram.png      - System context and external dependencies" -ForegroundColor White
Write-Host "2. 02-container-diagram.png    - Main containers and technology choices" -ForegroundColor White
Write-Host "3. 03-component-diagram.png    - Detailed component interactions" -ForegroundColor White
Write-Host "4. 04-deployment-diagram.png   - Infrastructure and deployment" -ForegroundColor White
Write-Host "5. 05-use-case-diagram.png     - User interactions and system functionality" -ForegroundColor White
Write-Host "6. 06-data-flow-diagram.png    - Data transformation through 8 factorial steps" -ForegroundColor White
Write-Host "7. 07-sequence-diagram.png     - Component interactions during pipeline execution" -ForegroundColor White
Write-Host "8. 08-l2-sidechain-architecture.png - L2 sidechain storage and mainnet rollup" -ForegroundColor White
Write-Host "9. 09-cross-references.png - Documentation relationships and navigation" -ForegroundColor White
Write-Host ""
Write-Host "📖 For more information, see README.md" -ForegroundColor Yellow 