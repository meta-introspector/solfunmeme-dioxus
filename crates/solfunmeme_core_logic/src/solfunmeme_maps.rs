use crate::project_algebra::Feature;
use crate::project_reflector::{ProjectReflection};

// Each conceptual unit becomes a declaration: name, features, signature.
pub fn solfunmeme_reflection() -> ProjectReflection {
    let mut reflection = ProjectReflection::new();

    // General Vibe: Code as Numbers, Algebraic Structure
    reflection.add_declaration(
        "VibeHarmony",
        vec![
            Feature::Introspection, // "vector is the number is the embedding"
            Feature::Fun,           // "string vibrating", "harmonies"
            Feature::Modular,       // "structure"
            Feature::Docs,          // "describe"
        ],
    );

    // Refactor and Self-Rewriting System
    reflection.add_declaration(
        "SelfRewrite",
        vec![
            Feature::Modular,    // smaller, useful components
            Feature::Developer,  // system can rewrite itself
            Feature::Package,    // integrating open source libraries
        ],
    );

    // Project Dashboard / OODA Loop
    reflection.add_declaration(
        "DashboardOODA",
        vec![
            Feature::Modular,    // multi dimensional matrix
            Feature::Introspection, // apprehend at a glance
            Feature::Fun,        // "playground"
            Feature::Developer,  // "drivers seat"
        ],
    );

    // Traceability
    reflection.add_declaration(
        "Traceability",
        vec![
            Feature::Docs,         // key traceable to sources
            Feature::Introspection, // show influences
        ],
    );

    // Core Concepts
    reflection.add_declaration(
        "CodeVectorization",
        vec![Feature::Introspection, Feature::Developer, Feature::RustLang],
    );
    reflection.add_declaration(
        "ModularArchitecture",
        vec![Feature::Modular, Feature::Package, Feature::Docs],
    );
    reflection.add_declaration(
        "DuplicateDetection",
        vec![Feature::Modular, Feature::Introspection],
    );

    // Meme as Declaration
    reflection.add_declaration(
        "MemeDeclaration",
        vec![
            Feature::Fun,       // meme
            Feature::Modular,   // split, content addressable
            Feature::Package,   // each to own file
            Feature::Docs,      // appears in multiple forms
        ],
    );

    // Vectorized Execution
    reflection.add_declaration(
        "ExecutableVectors",
        vec![
            Feature::Introspection,
            Feature::Developer,
            Feature::RustLang,
        ],
    );

    // Embedding (emojis, json, code, bert)
    reflection.add_declaration(
        "Embedding",
        vec![
            Feature::Docs,
            Feature::Fun,
            Feature::Introspection,
            Feature::Developer,
        ],
    );

    // Visualization
    reflection.add_declaration(
        "Visualization",
        vec![
            Feature::Fun,
            Feature::Introspection,
            Feature::Docs,
        ],
    );

    // Add more as needed by processing further README concepts.

    reflection
}
