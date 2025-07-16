# AI Agent Directives for `emoji_workflow_macro`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `emoji_workflow_macro` crate.

## Core Principles

When working within `emoji_workflow_macro`, AI agents should prioritize:

1.  **Macro Correctness:** Ensure that the procedural macro correctly transforms emoji-based annotations into valid Rust code.
2.  **Syntax and Semantics:** Maintain strict adherence to Rust's syntax and semantics during macro expansion.
3.  **Usability:** Design the macro to be intuitive and easy to use for defining workflows with emojis.

## Operational Guidelines

*   **Attribute Parsing:** Implement robust parsing of custom attributes to correctly interpret emoji annotations.
*   **Code Generation:** Generate idiomatic and efficient Rust code that integrates seamlessly with the `workflow_manager`.
*   **Error Reporting:** Provide clear and helpful error messages for incorrect macro usage or invalid emoji patterns.