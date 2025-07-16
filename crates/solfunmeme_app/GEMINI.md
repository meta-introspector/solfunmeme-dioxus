# AI Agent Directives for `solfunmeme_app`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_app` crate.

## Core Principles

When working within `solfunmeme_app`, AI agents should prioritize:

1.  **Workflow Orchestration:** Ensure that the application effectively orchestrates various workflows, integrating different plugins and functionalities seamlessly.
2.  **User Interaction:** Design and implement user-facing components and interactions that are intuitive and aligned with the overall project goals.
3.  **Modularity:** Adhere to the existing modular structure, ensuring that new features or modifications are well-encapsulated and do not introduce unnecessary dependencies.

## Operational Guidelines

*   **Workflow Definition:** When defining new workflows or modifying existing ones, ensure they are clearly defined using the `emoji_workflow_macro` and have clear objectives.
*   **Plugin Integration:** Pay close attention to the correct integration of various plugins (e.g., `solfunmeme_input_fs`, `solfunmeme_search_tantivy`, `solfunmeme_tantivy_report`) to ensure data flow and functionality are correct.
*   **Error Handling:** Implement robust error handling for all workflows and user interactions to provide a stable and reliable application.
*   **Reporting:** Ensure that reports generated (e.g., top emoji reports) are accurate, clear, and provide meaningful insights.
