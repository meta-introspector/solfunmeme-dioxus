# AI Agent Directives for `layered_nlp_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `layered_nlp_plugin` crate.

## Core Principles

When working within `layered_nlp_plugin`, AI agents should prioritize:

1.  **Modular NLP Processing:** Ensure that NLP tasks are broken down into distinct, manageable layers, allowing for flexible and extensible processing pipelines.
2.  **Interoperability:** Facilitate seamless data flow and communication between different NLP layers.
3.  **Performance:** Optimize the overall NLP pipeline for efficient processing of text data.

## Operational Guidelines

*   **Layer Definition:** Clearly define the responsibilities and interfaces of each NLP layer.
*   **Data Transformation:** Ensure correct data transformation and representation as data passes through different layers.
*   **Error Handling:** Implement robust error handling for each layer and the overall pipeline.
