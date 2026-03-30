# Documentation Cross-References

A Rust-based documentation cross-reference system for the Solfunmeme-Dioxus project that analyzes and manages relationships between all documentation elements including ontologies, READMEs, PlantUML diagrams, and code.

## Features

- **Comprehensive Analysis**: Analyzes cross-references between all documentation types
- **Multiple Output Formats**: Generates text, JSON, and Markdown reports
- **PlantUML Integration**: Creates visual diagrams of documentation relationships
- **Navigation Patterns**: Provides guided navigation for different user types
- **Validation**: Checks for broken links and orphaned documents
- **8D Riemann Manifold Orientation**: Orients input vectors into geometric manifold space
- **Clifford Algebra Integration**: Uses geometric algebra for manifold operations
- **Geometric Attention**: Applies attention mechanisms in Clifford algebra space
- **CLI Interface**: Easy-to-use command-line interface

## Installation

The crate is part of the Solfunmeme-Dioxus workspace. Build it with:

```bash
cargo build -p doc_cross_references
```

## Usage

### Generate Documentation Index

```bash
# Generate text index (default)
cargo run -p doc_cross_references -- index

# Generate JSON index
cargo run -p doc_cross_references -- index --format json --output docs/index.json

# Generate Markdown index
cargo run -p doc_cross_references -- index --format markdown --output docs/index.md
```

### Analyze Cross-References

```bash
# Analyze all cross-references
cargo run -p doc_cross_references -- analyze

# Generate analysis with PlantUML diagram
cargo run -p doc_cross_references -- analyze --diagram

# Specify output directory
cargo run -p doc_cross_references -- analyze --diagram --output-dir docs/analysis
```

### Generate PlantUML Diagram

```bash
# Generate cross-reference diagram
cargo run -p doc_cross_references -- diagram

# Specify output file
cargo run -p doc_cross_references -- diagram --output docs/uml/09-cross-references.puml
```

### Show Navigation Patterns

```bash
# Show navigation for new users (default)
cargo run -p doc_cross_references -- navigate

# Show navigation for developers
cargo run -p doc_cross_references -- navigate --user-type developer

# Show navigation for architects
cargo run -p doc_cross_references -- navigate --user-type architect

# Show navigation for system administrators
cargo run -p doc_cross_references -- navigate --user-type admin
```

### Validate Cross-References

```bash
# Validate all cross-references
cargo run -p doc_cross_references -- validate

# Check for broken links
cargo run -p doc_cross_references -- validate --check-links

# Check for orphaned documents
cargo run -p doc_cross_references -- validate --check-orphans

# Check both
cargo run -p doc_cross_references -- validate --check-links --check-orphans
```

### Orient Input Vector to 8D Riemann Manifold

```bash
# Orient reverse index into 8D Riemann manifold
cargo run -p doc_cross_references -- orient --input reverse_index.json --output manifold_points.json

# Generate orientation report
cargo run -p doc_cross_references -- orient --input reverse_index.json --output manifold_points.json --report
```

## Documentation Categories

The system organizes documentation into the following categories:

### 🎯 Systems Design
- Core design documents and principles
- 8 factorial steps concept
- Algorithm documentation
- Chat system design
- Gemini integration

### 🏗️ Architecture
- System architecture overview
- Component descriptions
- CLI tools documentation
- Development setup guides

### 📊 UML Diagrams
- Visual architecture representations
- Context, container, component diagrams
- Deployment and use case diagrams
- Data flow and sequence diagrams
- L2 sidechain architecture

### 🧠 Ontologies
- Semantic models and relationships
- Project ontology (Turtle format)
- Solfunmeme concepts (JSON-LD)
- Introspection ontology
- ZOS CLI ontology

### ⚙️ Workflow & Tools
- Automated workflow implementation
- Makefile targets
- Workflow visualizations
- Quick reference guides

### 💻 Code Implementation
- Source code and implementation
- Cargo workspace configuration
- Architectural crates
- Test suites

## Navigation Patterns

The system provides guided navigation patterns for different user types:

### New Users
1. **Systems Design** - High-level understanding
2. **Context Diagram** - System boundaries
3. **Architecture** - Detailed architecture
4. **Quick Reference** - Practical commands

### Developers
1. **Architecture** - Component understanding
2. **Component Diagram** - Implementation details
3. **Cargo.toml** - Crate dependencies
4. **Makefile** - Development workflows

### Architects
1. **Systems Design** - Design principles
2. **UML Diagrams** - All architectural views
3. **Ontologies** - Semantic models
4. **L2 Architecture** - Sidechain design

### System Administrators
1. **Deployment Diagram** - Infrastructure
2. **Development Setup** - Setup procedures
3. **Makefile** - Deployment commands
4. **Quick Reference** - Operational commands

## Cross-Reference Types

The system tracks various types of cross-references:

- **Implements** - Code implements design
- **Visualizes** - Diagram visualizes concept
- **Defines** - Document defines concept
- **References** - Document references another
- **Models** - Ontology models concept
- **Documents** - Document describes system
- **Influences** - Design influences implementation
- **Drills Down** - Hierarchical relationship
- **Shows** - Diagram shows relationship
- **Explains** - Document explains concept

## Output Files

The system generates several types of output files:

- **Text Index** - Human-readable documentation index
- **JSON Index** - Machine-readable documentation structure
- **Markdown Index** - Formatted documentation index
- **PlantUML Diagram** - Visual cross-reference diagram
- **Analysis Report** - Detailed cross-reference analysis
- **Validation Report** - Broken links and orphaned documents

## Integration

This crate integrates with the existing Solfunmeme-Dioxus documentation system:

- **Makefile Integration** - Can be called from Makefile targets
- **UML Diagram Generation** - Updates existing PlantUML diagrams
- **Documentation Validation** - Ensures documentation consistency
- **CLI Tool Integration** - Part of the ZOS CLI ecosystem

## Development

### Building

```bash
# Build the crate
cargo build -p doc_cross_references

# Run tests
cargo test -p doc_cross_references

# Check documentation
cargo doc -p doc_cross_references
```

### Adding New Documentation

To add new documentation to the cross-reference system:

1. Update the `DocumentationIndex::initialize_documents()` method
2. Add appropriate cross-references in `establish_cross_references()`
3. Update navigation patterns if needed
4. Run validation to ensure consistency

### Extending the System

The system is designed to be extensible:

- Add new document categories in `DocumentCategory` enum
- Add new reference types in `ReferenceType` enum
- Create new navigation patterns for different user types
- Extend validation rules for specific requirements

## License

This crate is part of the Solfunmeme-Dioxus project and is licensed under the same terms as the main project. 