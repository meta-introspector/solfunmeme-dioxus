# AI Agent Directives for `git_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `git_plugin` crate.

## Core Principles

When working within `git_plugin`, AI agents should prioritize:

1.  **Git Operations Integrity:** Ensure that all Git operations performed through this plugin are robust, reliable, and maintain the integrity of the Git repository.
2.  **Security:** Handle Git credentials and sensitive repository information securely, avoiding any exposure.
3.  **Efficiency:** Optimize Git operations for performance, especially when dealing with large repositories or frequent interactions.

## Operational Guidelines

*   **Command Execution:** When executing Git commands, use the appropriate Rust libraries or safe shell command wrappers to prevent command injection vulnerabilities.
*   **Error Handling:** Implement comprehensive error handling for all Git operations, providing clear and actionable feedback in case of failures.
*   **Configuration:** Be mindful of Git configurations (e.g., user.name, user.email) and ensure they are handled appropriately.