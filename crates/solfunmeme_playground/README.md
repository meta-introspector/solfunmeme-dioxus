# `solfunmeme_playground`

This crate provides the interactive playground environment for the Solfunmeme project, offering experimental features, testing capabilities, and educational tools for exploring the Code-Math Manifold.

## Purpose

It serves as a sandbox environment where users can experiment with various features of the Solfunmeme ecosystem, visualize data, test algorithms, and interact with different components in a dynamic and responsive interface.

## Core Functionalities

### Interactive Applications
- **Main Playground App**: Central playground interface with navigation and component integration
- **Test Applications**: Various test apps for validating functionality and performance
- **Coverage Analysis**: Code coverage visualization and analysis tools
- **Performance Charts**: Performance monitoring and visualization components

### Experimental Features
- **BERT Testing**: BERT model integration and testing interface
- **Rust Parser**: Rust code parsing and analysis tools
- **Orbital Simulations**: Orbital mechanics visualization and simulation
- **Monster Meta Meme**: Advanced meme manipulation and visualization
- **Wikidata Integration**: Wikidata data exploration and meme generation

### Data Processing
- **Document Cleaner**: Document processing and cleaning utilities
- **Markdown Processor**: Markdown parsing and rendering tools
- **Embedding Tools**: Text embedding generation and visualization
- **Zip Utilities**: File compression and extraction tools

### Micro-Component Protocol
- **MCP Integration**: Micro-Component Protocol implementation and testing

## Usage (Conceptual)

```rust
use dioxus::prelude::*;
use solfunmeme_playground::{
    app::PlaygroundApp,
    test_app::TestApp,
    orbits::OrbitalSimulation,
    wikidata::WikidataExplorer,
};

fn main() {
    launch(PlaygroundApp);
}
``` 