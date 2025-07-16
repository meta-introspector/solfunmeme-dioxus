# AI Agent Directives for `vibrato_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `vibrato_plugin` crate.

## Core Principles

When working within `vibrato_plugin`, AI agents should prioritize:

1.  **Accurate Tokenization:** Ensure precise and efficient Japanese morphological analysis and tokenization using Vibrato.
2.  **Performance:** Optimize the tokenization process for speed and resource usage.
3.  **Dictionary Management:** Facilitate effective management and application of Vibrato dictionaries.

## Operational Guidelines

*   **Text Encoding:** Handle text encoding (e.g., UTF-8) correctly for Japanese text.
*   **Dictionary Loading:** Ensure proper loading and initialization of Vibrato dictionaries.
*   **Error Handling:** Provide robust error handling for tokenization failures or dictionary issues.
