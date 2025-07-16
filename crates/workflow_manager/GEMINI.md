# AI Agent Directives for `workflow_manager`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `workflow_manager` crate.

## Core Principles

When working within `workflow_manager`, AI agents should prioritize:

1.  **Workflow Definition and Execution:** Ensure robust mechanisms for defining, registering, and executing various workflows within the application.
2.  **Modularity and Extensibility:** Design the manager to be highly modular, allowing for easy addition of new workflows and integration with different components.
3.  **State Management:** Maintain consistent and reliable state management for ongoing workflows.

## Operational Guidelines

*   **Workflow Registration:** Ensure that workflows are correctly registered and discoverable by the application.
*   **Dependency Management:** Handle workflow dependencies effectively to ensure proper execution order.
*   **Error Handling:** Implement comprehensive error handling for workflow execution, providing clear feedback in case of failures.