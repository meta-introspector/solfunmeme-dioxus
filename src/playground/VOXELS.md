# Playground Voxels — Semantic Index

Each playground module is a **voxel**: a self-contained idea with coordinates in the project's concept space.

## Voxel Map

| Voxel | File | Lines | Category | Status | Description |
|-------|------|-------|----------|--------|-------------|
| V01-UI-NICE | solfunnice.rs | 1412 | ui/animation | 🟡 Draft | Animated Dioxus UI with motion, timers, emoji effects |
| V02-TEST-CASES | test_app.rs | 1197 | testing | 🟡 Draft | Use cases from coverage report: wallet, clusters, plugins |
| V03-COMPONENTS | test_components.rs | 1083 | ui/registry | 🟡 Draft | Component registry with password manager, builder |
| V04-MCP-TOOLS | mcp.rs | 834 | tools/mcp | 🟢 Active | MCP tool orchestration surface, Lean4 styles |
| V05-ZOS-PROTO | solfunmeme.rs | 825 | core/zos | 📦 Archive | Early ZOS CLI prototype (superseded by src/bin/zos/) |
| V06-COVERAGE | coverage_app.rs | 770 | testing/coverage | 🟡 Draft | Coverage viewer UI |
| V07-RUST-PARSER | rust_parser.rs | 609 | tools/parser | 🟢 Active | Rust AST parser via syn + syn_serde |
| V08-MONSTER | monster_meta_meme.rs | 395 | math/monster | 🟢 Active | Monster Group meta-meme visualization |
| V09-BERT-WASM | rust_bert_wasm.rs | 373 | ml/bert | 🟡 Draft | WASM BERT embeddings, sentiment, classification |
| V10-ORBITS | orbits.rs | 303 | math/orbital | 🟢 Active | Orbital simulation with emoji matrix rollup |
| V11-CHARTS | performance_charts.rs | 195 | ui/charts | 🟢 Active | Bar/Line/Pie charts via dioxus-charts |
| V12-BERT-TEST | bert_test.rs | 138 | ml/bert | 🟡 Draft | BERT embedder test harness |
| V13-APP-SHELL | app.rs | 102 | ui/shell | 📦 Archive | Old app shell (superseded by src/main.rs) |
| V14-ANIMATIONS | test_animations.rs | 76 | ui/animation | 🟡 Draft | Motion/easing animation tests |
| V15-WIKIDATA | wikidata.rs | 29 | data/wikidata | 🔴 Stub | Wikidata SPARQL graph fetch (commented out) |
| V16-ZIP | zip.rs | 28 | tools/zip | 🔴 Stub | ZIP export of code snippets |
| V17-EMBEDDING | embedding.rs | 11 | ml/embedding | 🔴 Stub | Embedding pipeline imports (commented out) |
| V18-EMOJIS | test_emojis.rs | 6 | ui/emoji | 📦 Archive | Re-export of component_builder_lib |
| V19-DOC-CLEAN | doc_cleaner.rs | 6 | tools/docs | 🔴 Stub | Document cleaning placeholder |
| V20-MARKDOWN | markdown_processor.rs | 1 | tools/markdown | 🔴 Stub | Empty |
| V21-MOD | mod.rs | 38 | — | — | Module declarations |

## Categories

- **ui/**: Dioxus components, animations, charts
- **ml/**: BERT, embeddings, sentiment
- **math/**: Monster Group, orbits, Gödel
- **tools/**: MCP, parser, zip, docs
- **core/**: ZOS, app shell
- **data/**: Wikidata, external sources
- **testing/**: Coverage, test cases

## Actions

### Keep (active voxels → promote to crates/)
- V04-MCP-TOOLS, V07-RUST-PARSER, V08-MONSTER, V10-ORBITS, V11-CHARTS

### Merge (draft voxels → consolidate)
- V01+V14 → `ui_animations` crate
- V09+V12+V17 → `ml_bert` crate
- V02+V06 → `testing` crate
- V03+V18 → `component_registry` (already exists in crates/)

### Archive (superseded)
- V05-ZOS-PROTO, V13-APP-SHELL → move to `archive/playground/`

### Delete (empty stubs)
- V15, V16, V19, V20 → remove
