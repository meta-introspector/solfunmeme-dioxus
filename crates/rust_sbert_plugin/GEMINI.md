# AI Agent Directives for `rust_sbert_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `rust_sbert_plugin` crate.

## Core Principles

When working within `rust_sbert_plugin`, AI agents should prioritize:

1.  **Accurate Sentence Embeddings:** Ensure the accurate generation of sentence embeddings using SBERT models.
2.  **Performance:** Optimize the embedding process for speed and efficiency.
3.  **Model Integration:** Facilitate seamless integration and usage of pre-trained SBERT models.

## Operational Guidelines

*   **Model Loading:** Handle the loading and management of SBERT models correctly.
*   **Text Preprocessing:** Implement necessary text preprocessing steps before feeding text to the SBERT model.
*   **Error Handling:** Provide robust error handling for model loading, inference, and other operations.
