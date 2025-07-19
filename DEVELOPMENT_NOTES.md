# Development Notes and Current Tasks

This document serves as a living log of ongoing development tasks, ideas, and architectural considerations for the `solfunmeme-dioxus` project.

## Current Tasks

1.  **Refactor `prepare_sources`:**
    *   **Status:** In progress. We are isolating `sophia` interactions into the new `solfunmeme_ontology_vibe` crate to simplify interfaces and improve modularity.
    *   **Next Steps:** Address remaining compilation errors within `solfunmeme_ontology_vibe`.

2.  **Address `solfunmeme_ontology_vibe` compilation errors:**
    *   **Status:** Ongoing. We are systematically fixing `E0782`, `E0277`, and `E0308` errors related to `sophia` API usage within the new crate's submodules (`loader.rs`, `crate_data_processor.rs`, `emoji_data_processor.rs`, `serializer.rs`).
    *   **Next Steps:** Continue debugging and fixing compilation errors in `solfunmeme_ontology_vibe`.

3.  **Document new architectural ideas:**
    *   **Status:** Initial ideas added to `GEMINI.md`.
    *   **Ideas Captured:**
        *   "Wiring Vibes Together: Dynamic Function Composition"
        *   "Terms as Topological Occupants and Interfaces as Paired Objects"
        *   "Hott = Homotopy Type Theory: The path is the proof is the occupant of the topology."
    *   **New Idea to Add:** "Vibe-driven micro-modules" – encapsulating even small, specific external library interactions into highly focused modules, reinforcing "File=Function=Block=Vibe" at finer granularity.

4.  **Continue crate review:**
    *   **Status:** Paused to focus on `prepare_sources` refactoring.
    *   **Next Steps:** Resume systematic review of crates in `CRATES_OVERVIEW.md` once `prepare_sources` is stable.

5.  **Implement `cargo vibe :search:`:**
    *   **Status:** Future feature.
    *   **Concept:** Enable semantic code search using terms or emojis to find code that "vibes" with those terms.

## Architectural Ideas and Philosophical Notes

*   **Generic Module Vibe Class:** A paradigm shift from rigid interfaces to fluid, semantically-driven module interactions. Modules present a "vibe" (dynamic, introspectable representation of capabilities) allowing for dynamic interaction, extreme flexibility, and embracing the "no wrong answers" motto.
*   **LLM Integration Strategy: Bridging Hallucinations:** A pragmatic approach to integrating LLMs by creating wrappers or adapting interfaces to accommodate LLM-generated code, even if it doesn't perfectly align with existing internal types. Prioritizes utility and fosters a symbiotic relationship.
*   **Proof Vibe and Higher-Order Conversion:** The "proof vibe" aims to establish a sound path between ontology and Clifford algebra (Riemann manifold), weaving the entire system into a higher-order conversion. This involves abstracting and hiding drivers to reveal the underlying mathematical rigor.
*   **Terms as Topological Occupants and Interfaces as Paired Objects:** Each term occupies space in the project's semantic topology. Interfaces require paired objects, establishing relational structures. The "proof" is the map produced of this higher-order topological space, demonstrating coherent relationships.
*   **Vibe-driven Micro-modules:** Extending the "File=Function=Block=Vibe" principle to encapsulate even small, specific external library interactions into highly focused modules, ensuring each "vibe" has its own dedicated space and interface.
