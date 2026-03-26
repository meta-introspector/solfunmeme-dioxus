# Solfunmeme Dioxus Makefile

.PHONY: test coverage clean build doc-tests integration-tests unit-tests

# Default target
all: test coverage

# Run all tests
test: unit-tests integration-tests doc-tests
	@echo "✅ All tests completed"

# Run unit tests only
unit-tests:
	@echo "🧪 Running unit tests..."
	cargo test --lib

# Run integration tests
integration-tests:
	@echo "🔗 Running integration tests..."
	cargo test --test integration_tests

# Generate and run documentation tests
doc-tests:
	@echo "📚 Generating documentation tests..."
	cargo run --bin doc_test_generator
	@echo "📖 Running documentation tests..."
	cargo test --test doc_tests

# Run comprehensive test runner
test-runner:
	@echo "🚀 Running comprehensive test suite..."
	cargo run --bin test_runner

# Generate code coverage report
coverage:
	@echo "📊 Generating code coverage report..."
	cargo tarpaulin --out Html --output-dir coverage/
	@echo "📈 Coverage report generated in coverage/"

# Alternative coverage with llvm-cov
coverage-llvm:
	@echo "📊 Generating coverage with llvm-cov..."
	set RUSTFLAGS=-C instrument-coverage && cargo build
	cargo llvm-cov --html --output-dir coverage-llvm/

# Clean build artifacts and coverage reports
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	if exist coverage rmdir /s /q coverage
	if exist coverage-llvm rmdir /s /q coverage-llvm
	if exist tests\doc_tests.rs del tests\doc_tests.rs

# Build the project
build:
	@echo "🔨 Building project..."
	cargo build

# Build for release
build-release:
	@echo "🚀 Building release version..."
	cargo build --release

# Run clippy for linting
lint:
	@echo "🔍 Running clippy linter..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "✨ Formatting code..."
	cargo fmt

# Check formatting
fmt-check:
	@echo "🔍 Checking code formatting..."
	cargo fmt -- --check

# Run security audit
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open

# Run benchmarks (if any)
bench:
	@echo "⚡ Running benchmarks..."
	cargo bench

# Full CI pipeline
ci: fmt-check lint test coverage audit
	@echo "✅ CI pipeline completed successfully"

# Development setup
dev-setup:
	@echo "🛠️  Setting up development environment..."
	rustup component add clippy rustfmt
	cargo install cargo-tarpaulin cargo-audit cargo-llvm-cov
	@echo "✅ Development environment ready"

# Help target
help:
	@echo "Available targets:"
	@echo "  test           - Run all tests"
	@echo "  unit-tests     - Run unit tests only"
	@echo "  integration-tests - Run integration tests"
	@echo "  doc-tests      - Generate and run doc tests"
	@echo "  test-runner    - Run comprehensive test suite"
	@echo "  coverage       - Generate code coverage report"
	@echo "  coverage-llvm  - Generate coverage with llvm-cov"
	@echo "  build          - Build the project"
	@echo "  build-release  - Build release version"
	@echo "  lint           - Run clippy linter"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check code formatting"
	@echo "  audit          - Run security audit"
	@echo "  docs           - Generate documentation"
	@echo "  clean          - Clean build artifacts"
	@echo "  ci             - Run full CI pipeline"
	@echo "  dev-setup      - Set up development environment"
	@echo "  help           - Show this help message"

# ── Nix build ─────────────────────────────────────────────────────

.PHONY: nix-check nix-build nix-serve nix-test-headless deploy-local

nix-check:
	nix develop --command cargo check

nix-build:
	nix develop --command cargo build --release --target wasm32-unknown-unknown

nix-serve:
	nix develop --command dx serve

deploy-local:
	nix develop --command dx build --release
	@echo "✓ Built to dist/"
	@echo "  Serve: python3 -m http.server 8888 -d dist"

# ── Headless browser tests ────────────────────────────────────────

test-headless: deploy-local
	@echo "🧪 Running headless browser tests..."
	nix develop --command bash -c '\
		python3 -m http.server 8888 -d dist & \
		SERVER_PID=$$!; \
		sleep 2; \
		echo "Testing /dao route..."; \
		$$CHROME_BIN --headless --no-sandbox --disable-gpu \
			--dump-dom http://localhost:8888/dao 2>/dev/null | head -20; \
		echo "Testing /paste route..."; \
		$$CHROME_BIN --headless --no-sandbox --disable-gpu \
			--dump-dom http://localhost:8888/paste 2>/dev/null | head -20; \
		echo "Testing /p2p route..."; \
		$$CHROME_BIN --headless --no-sandbox --disable-gpu \
			--dump-dom http://localhost:8888/p2p 2>/dev/null | head -20; \
		kill $$SERVER_PID 2>/dev/null; \
		echo "✅ Headless tests complete"'

# ── GitHub Actions ────────────────────────────────────────────────

ci: nix-check test nix-build
	@echo "✅ CI complete"

# ── Deploy targets ────────────────────────────────────────────────

SHARDS := 71
DOCS := docs

.PHONY: dx-release shards deploy-cf deploy-hf deploy-vercel deploy-all

# Build release WASM (no base_path for static deploys)
dx-release:
	sed -i 's/^base_path = "dioxus"/# base_path = "dioxus"/' Dioxus.toml
	nix develop --command dx build --release --platform web
	sed -i 's/^# base_path = "dioxus"/base_path = "dioxus"/' Dioxus.toml
	@echo "✓ Release built"

# Split WASM into 71 Gandalf shards + manifest
shards: dx-release
	rm -rf $(DOCS)/*
	cp -r target/dx/solfunmeme-dioxus/release/web/public/assets $(DOCS)/
	WASM=$$(ls $(DOCS)/assets/*_bg*.wasm | head -1); \
	mkdir -p $(DOCS)/assets/shards; \
	TOTAL=$$(wc -c < "$$WASM"); \
	split -b $$(( ($$TOTAL + $(SHARDS) - 1) / $(SHARDS) )) -d -a 2 "$$WASM" $(DOCS)/assets/shards/shard_; \
	for f in $(DOCS)/assets/shards/shard_[0-9]*; do mv "$$f" "$${f}.wasm"; done; \
	rm -f "$$WASM"; \
	python3 -c "import hashlib,json,os; \
	d='$(DOCS)/assets/shards'; \
	s=[{'id':i,'size':os.path.getsize(os.path.join(d,f'shard_{i:02d}.wasm')),'hash':hashlib.sha256(open(os.path.join(d,f'shard_{i:02d}.wasm'),'rb').read()).hexdigest()} for i in range($(SHARDS))]; \
	json.dump({'total_shards':$(SHARDS),'total_bytes':sum(x['size'] for x in s),'shards':s},open(os.path.join(d,'manifest.json'),'w'))"
	cp assets/shard-loader.js $(DOCS)/assets/ 2>/dev/null || true
	@echo "✓ $(SHARDS) shards created"

# Cloudflare Pages deploy
deploy-cf: shards
	nix develop --command npx wrangler pages deploy $(DOCS)/ \
		--project-name=solfunmeme-dioxus --branch=main
	@echo "✓ Deployed to Cloudflare Pages"

# HuggingFace Space deploy
deploy-hf: shards
	python3 -c "from huggingface_hub import HfApi; \
	HfApi().upload_folder(folder_path='$(DOCS)',repo_id='introspector/solfunmeme-dioxus',repo_type='space', \
	commit_message='Deploy $(SHARDS)-shard WASM')"
	@echo "✓ Deployed to HuggingFace"

# Push docs/ for Vercel (auto-deploys on push)
deploy-vercel: shards
	git add $(DOCS)/ vercel.json
	git commit -m "Deploy $(SHARDS)-shard WASM to Vercel" || true
	git push jmikedupont2 HEAD:main
	@echo "✓ Pushed to Vercel"

# All platforms
deploy-all: deploy-cf deploy-hf deploy-vercel
	@echo "✓ Deployed to CF Pages + HuggingFace + Vercel"
