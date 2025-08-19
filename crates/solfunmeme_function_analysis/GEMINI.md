# AI Agent Directives for `solfunmeme_function_analysis`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_function_analysis` crate.

## Core Principles

When working within `solfunmeme_function_analysis`, AI agents should prioritize:

1.  **Centralized Data Models:** This crate now serves as the primary location for core data structures related to code analysis, such as `CodeChunk`, `AnalyzedFunction`, and `ClosestEmojiInfo`. Ensure all modifications and additions adhere to this principle, preventing data model fragmentation across the project.
2.  **Semantic Richness:** Data models should capture and represent the semantic meaning of code effectively, facilitating deeper analysis and understanding.
3.  **Consistency and Clarity:** Maintain consistent naming conventions, clear data definitions, and comprehensive documentation for all structures.

## Operational Guidelines

*   **Data Model Definition:** When defining new data models or modifying existing ones, ensure they are generic enough to be reusable across different analysis contexts but specific enough to capture necessary details.
*   **Serialization:** All data models should be easily serializable and deserializable (e.g., using `serde`) to support inter-process communication and persistence.
*   **Type Safety:** Leverage Rust's type system to enforce data integrity and prevent common errors.
*   **Backward Compatibility:** When making changes to existing data models, consider the impact on downstream crates and strive for backward compatibility where feasible.
*   **Documentation:** Provide clear and concise documentation for each struct and its fields, explaining their purpose and usage.
