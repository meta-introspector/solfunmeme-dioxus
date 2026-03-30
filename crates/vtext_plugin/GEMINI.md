# AI Agent Directives for `vtext_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `vtext_plugin` crate.

## Core Principles

When working within `vtext_plugin`, AI agents should prioritize:

1.  **Text Vectorization Accuracy:** Ensure accurate and efficient conversion of text into numerical vector representations using vtext.
2.  **Performance:** Optimize the plugin for fast text processing and vectorization.
3.  **Feature Engineering:** Facilitate effective feature engineering from text data for downstream machine learning tasks.

## Operational Guidelines

*   **Text Preprocessing:** Handle text preprocessing steps (e.g., tokenization, stemming) correctly before vectorization.
*   **Vectorization Models:** Apply appropriate vtext vectorization models (e.g., TF-IDF, CountVectorizer) based on the use case.
*   **Error Handling:** Implement robust error handling for vtext operations.
