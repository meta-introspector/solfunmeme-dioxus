# Solfunmeme-Dioxus System Workflow
# Based on Systems Design: 8 Factorial Steps via LLMs with Autopoetic Rewrites
# 
# This Makefile orchestrates the complete workflow from source code to 
# semantic analysis, emoji embeddings, and blockchain integration.

.PHONY: help all setup build test clean deploy nix nix-shell
.PHONY: index analyze extract embed search
.PHONY: rdf semantic blockchain wallet
.PHONY: ui components playground tools
.PHONY: plugins workflow pipeline

# Default target
all: setup build index analyze

# Nix build
nix:
	nix build

# Nix dev shell
nix-shell:
	nix develop

# Show help with system overview
help:
	@echo "🌐 Solfunmeme-Dioxus System Workflow"
	@echo "====================================="
	@echo ""
	@echo "🎯 Core Workflow (8 Factorial Steps):"
	@echo "  1. Source Preparation (prepare_sources)"
	@echo "  2. Code Extraction (solfunmeme_extractor)"
	@echo "  3. Function Analysis (solfunmeme_function_analysis)"
	@echo "  4. Embedding Generation (solfunmeme_embedding)"
	@echo "  5. Semantic Indexing (solfunmeme_search_tantivy)"
	@echo "  6. RDF Processing (rdf_processing_lib)"
	@echo "  7. Blockchain Integration (solana_integration_lib)"
	@echo "  8. UI Rendering (solfunmeme_views)"
	@echo ""
	@echo "🔧 Available Targets:"
	@echo "  setup      - Initialize development environment"
	@echo "  build      - Build all crates and dependencies"
	@echo "  index      - Index codebase for search and analysis"
	@echo "  analyze    - Run comprehensive code analysis"
	@echo "  extract    - Extract code chunks and functions"
	@echo "  embed      - Generate BERT embeddings and emoji vectors"
	@echo "  search     - Search indexed codebase"
	@echo "  rdf        - Generate RDF triples and semantic data"
	@echo "  semantic   - Process semantic relationships"
	@echo "  blockchain - Setup Solana integration"
	@echo "  wallet     - Configure wallet integration"
	@echo "  ui         - Build UI components and views"
	@echo "  components - Build component system"
	@echo "  playground - Setup interactive playground"
	@echo "  tools      - Build CLI tools"
	@echo "  plugins    - Build and load plugins"
	@echo "  workflow   - Run workflow manager"
	@echo "  pipeline   - Execute processing pipeline"
	@echo "  deploy     - Deploy to production"
	@echo "  clean      - Clean build artifacts"
	@echo ""
	@echo "🚀 Quick Start: make setup && make all"

# =============================================================================
# SETUP & BUILD
# =============================================================================

# Initialize development environment
setup:
	@echo "🔧 Setting up Solfunmeme-Dioxus development environment..."
	cargo install wasm-pack
	cargo install trunk
	rustup target add wasm32-unknown-unknown
	@echo "✅ Development environment ready"

# Build all crates and dependencies
build:
	@echo "🏗️ Building Solfunmeme-Dioxus system..."
	cargo build --workspace
	@echo "✅ Build complete"

# Build for web deployment
build-web:
	@echo "🌐 Building for web deployment..."
	wasm-pack build --target web
	@echo "✅ Web build complete"

# =============================================================================
# CORE DATA PROCESSING PIPELINE
# =============================================================================

# Index codebase for search and analysis
index:
	@echo "📚 Indexing codebase for search and analysis..."
	cargo run --bin full_indexer_cli -- build
	@echo "✅ Codebase indexed"

# Run comprehensive code analysis
analyze:
	@echo "🔍 Running comprehensive code analysis..."
	cargo run --bin codebase_analyzer_cli -- stats
	cargo run --bin codebase_analyzer_cli -- word-freq 50
	cargo run --bin codebase_analyzer_cli -- emoji-freq 20
	@echo "✅ Analysis complete"

# Extract code chunks and functions
extract:
	@echo "📝 Extracting code chunks and functions..."
	cargo run --bin prepare_sources
	cargo run --bin solfunmeme_extractor
	@echo "✅ Code extraction complete"

# Generate BERT embeddings and emoji vectors
embed:
	@echo "🧠 Generating BERT embeddings and emoji vectors..."
	cargo run --bin solfunmeme_embedding
	@echo "✅ Embeddings generated"

# Search indexed codebase
search:
	@echo "🔎 Searching indexed codebase..."
	cargo run --bin codebase_analyzer_cli -- search "function" 10
	@echo "✅ Search complete"

# =============================================================================
# SEMANTIC & RDF PROCESSING
# =============================================================================

# Generate RDF triples and semantic data
rdf:
	@echo "🔗 Generating RDF triples and semantic data..."
	cargo run --bin rdf_output
	@echo "✅ RDF generation complete"

# Process semantic relationships
semantic:
	@echo "🧩 Processing semantic relationships..."
	cargo run --bin rdf_processing_lib
	@echo "✅ Semantic processing complete"

# =============================================================================
# BLOCKCHAIN & SOLANA INTEGRATION
# =============================================================================

# Setup Solana integration
blockchain:
	@echo "⛓️ Setting up Solana blockchain integration..."
	cargo run --bin solana_integration_lib
	@echo "✅ Blockchain integration ready"

# Configure wallet integration
wallet:
	@echo "💰 Configuring wallet integration..."
	cargo run --bin solfunmeme_wallet_integration
	@echo "✅ Wallet integration ready"

# =============================================================================
# UI & COMPONENTS
# =============================================================================

# Build UI components and views
ui:
	@echo "🎨 Building UI components and views..."
	cargo run --bin solfunmeme_views
	@echo "✅ UI components built"

# Build component system
components:
	@echo "🧩 Building component system..."
	cargo run --bin component_builder_lib
	cargo run --bin component_registry_lib
	cargo run --bin component_emoji_lib
	@echo "✅ Component system built"

# Setup interactive playground
playground:
	@echo "🎮 Setting up interactive playground..."
	cargo run --bin solfunmeme_playground
	@echo "✅ Playground ready"

# =============================================================================
# TOOLS & CLI
# =============================================================================

# Build CLI tools
tools:
	@echo "🛠️ Building CLI tools..."
	cargo run --bin solfunmeme_tools
	cargo run --bin plan_cli
	@echo "✅ CLI tools built"

# Build and load plugins
plugins:
	@echo "🔌 Building and loading plugins..."
	cargo run --bin workflow_manager
	@echo "✅ Plugins loaded"

# Run workflow manager
workflow:
	@echo "⚙️ Running workflow manager..."
	cargo run --bin workflow_manager -- execute
	@echo "✅ Workflow executed"

# Execute processing pipeline
pipeline:
	@echo "🔄 Executing processing pipeline..."
	@echo "Step 1: Source preparation..."
	cargo run --bin prepare_sources
	@echo "Step 2: Code extraction..."
	cargo run --bin solfunmeme_extractor
	@echo "Step 3: Function analysis..."
	cargo run --bin solfunmeme_function_analysis
	@echo "Step 4: Embedding generation..."
	cargo run --bin solfunmeme_embedding
	@echo "Step 5: Semantic indexing..."
	cargo run --bin solfunmeme_search_tantivy
	@echo "Step 6: RDF processing..."
	cargo run --bin rdf_processing_lib
	@echo "Step 7: Blockchain integration..."
	cargo run --bin solana_integration_lib
	@echo "Step 8: UI rendering..."
	cargo run --bin solfunmeme_views
	@echo "✅ Pipeline complete"

# =============================================================================
# ZOS CLI INTEGRATION
# =============================================================================

# Start ZOS interactive CLI
zos:
	@echo "🚀 Starting ZOS interactive CLI..."
	cargo run --bin zos-driver interactive

# Run ZOS pipeline command
zos-pipeline:
	@echo "🔄 Running ZOS pipeline..."
	cargo run --bin zos-driver pipeline "function" --sort --uniq --head 20

# =============================================================================
# DEPLOYMENT & CLEANUP
# =============================================================================

# Deploy to production
deploy:
	@echo "🚀 Deploying to production..."
	cargo build --release
	@echo "✅ Deployment ready"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf target/
	rm -rf tmp/
	@echo "✅ Clean complete"

# =============================================================================
# DEVELOPMENT WORKFLOWS
# =============================================================================

# Development workflow: build, test, analyze
dev: build test analyze

# Test the system
test:
	@echo "🧪 Running tests..."
	cargo test --workspace
	@echo "✅ Tests complete"

# Watch for changes and rebuild
watch:
	@echo "👀 Watching for changes..."
	cargo watch -x check -x test -x run

# =============================================================================
# SPECIALIZED WORKFLOWS
# =============================================================================

# Emoji workflow processing
emoji-workflow:
	@echo "😊 Processing emoji workflows..."
	cargo run --bin emoji_lang_plugin
	cargo run --bin emoji_workflow_macro
	@echo "✅ Emoji workflow complete"

# Clifford algebra operations
clifford:
	@echo "🔢 Running Clifford algebra operations..."
	cargo run --bin solfunmeme_clifford
	@echo "✅ Clifford operations complete"

# Orbital simulation
orbital:
	@echo "🛸 Running orbital simulation..."
	cargo run --bin orbital_sim_lib
	@echo "✅ Orbital simulation complete"

# NLP processing
nlp:
	@echo "📝 Running NLP processing..."
	cargo run --bin layered_nlp_plugin
	cargo run --bin keyword_extraction_rs_plugin
	cargo run --bin bm25_plugin
	@echo "✅ NLP processing complete"

# =============================================================================
# SYSTEM DIAGNOSTICS
# =============================================================================

# Show system status
status:
	@echo "📊 Solfunmeme-Dioxus System Status"
	@echo "=================================="
	@echo "Workspace members:"
	@cargo metadata --format-version 1 | jq -r '.workspace_members[]' | head -20
	@echo ""
	@echo "Build status:"
	@cargo check --quiet && echo "✅ Build ready" || echo "❌ Build issues found"
	@echo ""
	@echo "Index status:"
	@test -d "tmp/tantivy_index" && echo "✅ Index exists" || echo "❌ No index found"

# Show dependency graph
deps:
	@echo "📈 Dependency Graph"
	@echo "=================="
	cargo tree --workspace

# =============================================================================
# DOCUMENTATION
# =============================================================================

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --workspace --no-deps
	@echo "✅ Documentation generated"

# Open documentation
docs-open:
	@echo "📖 Opening documentation..."
	xdg-open target/doc/solfunmeme_dioxus/index.html 2>/dev/null || \
	open target/doc/solfunmeme_dioxus/index.html 2>/dev/null || \
	echo "Documentation available at target/doc/solfunmeme_dioxus/index.html"

# =============================================================================
# UTILITY TARGETS
# =============================================================================

# Show file structure
tree:
	@echo "🌳 Project Structure"
	@echo "==================="
	tree -I 'target|vendor|.git' -L 3

# Show recent changes
changes:
	@echo "📝 Recent Changes"
	@echo "================"
	git log --oneline -10

# Backup current state
backup:
	@echo "💾 Creating backup..."
	tar -czf backup-$(date +%Y%m%d-%H%M%S).tar.gz --exclude=target --exclude=vendor .
	@echo "✅ Backup created"

# =============================================================================
# SYSTEM INTEGRATION
# =============================================================================

# Full system integration test
integration-test:
	@echo "🔗 Running full system integration test..."
	make setup
	make build
	make index
	make analyze
	make extract
	make embed
	make search
	make rdf
	make semantic
	make blockchain
	make wallet
	make ui
	make components
	make playground
	make tools
	make plugins
	make workflow
	@echo "✅ Full system integration test complete"

# Quick system check
quick-check:
	@echo "⚡ Quick system check..."
	cargo check --workspace --quiet
	cargo test --workspace --quiet
	@echo "✅ Quick check complete"

# =============================================================================
# HELPERS
# =============================================================================

# Show this help
.PHONY: help-all
help-all: help
	@echo ""
	@echo "📋 Documentation Cross-References:"
	@echo "================================"
	@echo "• Systems Design: vendor/meta-meme.wiki/SystemsDesign.md"
	@echo "• Architecture: doc/architecture.md"
	@echo "• UML Diagrams: docs/uml/"
	@echo "• Cross-References: docs/uml/CROSS_REFERENCES.md"
	@echo "• Workflow Diagram: WORKFLOW_DIAGRAM.md"
	@echo "• Quick Reference: QUICK_REFERENCE.md"
	@echo "• Ontologies: ontologies/"
	@echo ""
	@echo "🎯 Documentation Navigation:"
	@echo "=========================="
	@echo "• Start with Systems Design for high-level understanding"
	@echo "• Use UML diagrams for visual architecture exploration"
	@echo "• Reference Cross-References for documentation relationships"
	@echo "• Use Quick Reference for practical commands"
	@echo "• Check ontologies for semantic models"
	@echo ""
	@echo "📋 Additional Information:"
	@echo "========================="
	@echo "• Systems Design: vendor/meta-meme.wiki/SystemsDesign.md"
	@echo "• Architecture: doc/architecture.md"
	@echo "• CLI Tools: doc/cli_tools_guide.md"
	@echo "• Main CLI: zos/src/main.rs"
	@echo ""
	@echo "🎯 Key Concepts:"
	@echo "==============="
	@echo "• 8 Factorial Steps: Autopoetic rewrites via LLMs"
	@echo "• Code-Math Manifold: Mathematical representation of code"
	@echo "• Emoji Vectors: BERT embeddings converted to emoji representations"
	@echo "• RDF Processing: Semantic data representation"
	@echo "• Solana Integration: Blockchain-based code storage"
	@echo "• Component System: Dynamic UI component management"
