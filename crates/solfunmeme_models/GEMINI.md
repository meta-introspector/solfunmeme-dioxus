# AI Agent Directives for `solfunmeme_models`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_models` crate.

## Core Principles

When working within `solfunmeme_models`, AI agents should prioritize:

1. **Type Safety**: Ensure all data models are well-typed and provide compile-time guarantees for data integrity.
2. **Consistency**: Maintain consistent naming conventions and structure across all model definitions.
3. **Serialization**: Design models to be easily serializable/deserializable for storage and transmission.
4. **Documentation**: Provide clear documentation for all model structures and their intended use cases.

## Operational Guidelines

* **Model Design**: When creating new models, consider their relationship to existing models and ensure they follow established patterns.
* **Validation**: Implement appropriate validation logic for model data, especially for blockchain-related structures.
* **Error Handling**: Use proper error types and provide meaningful error messages for model operations.
* **Performance**: Consider the performance implications of model structures, especially for frequently used types.
* **Compatibility**: Ensure models are compatible with both web and native environments where applicable.
* **Mathematical Accuracy**: When working with mathematical models (Clifford algebra, Lean), ensure mathematical correctness and precision. 