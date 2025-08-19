# AI Agent Directives for `solfunmeme_state`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_state` crate.

## Core Principles

When working within `solfunmeme_state`, AI agents should prioritize:

1. **State Consistency**: Ensure that state changes are atomic and maintain consistency across the application.
2. **Performance**: Optimize state management for performance, especially for large state objects and frequent updates.
3. **Persistence**: Implement reliable state persistence that handles errors gracefully and maintains data integrity.
4. **Reactivity**: Design state management to be reactive and efficient, minimizing unnecessary re-renders.

## Operational Guidelines

* **State Design**: When designing new state structures, consider their impact on performance and ensure they are serializable.
* **State Updates**: Implement state updates that are atomic and provide clear error handling for failed updates.
* **Persistence**: Ensure state persistence is reliable and handles edge cases like storage quota exceeded or network failures.
* **State Synchronization**: Implement proper synchronization mechanisms to keep state consistent across different parts of the application.
* **Memory Management**: Be mindful of memory usage, especially for large state objects, and implement appropriate cleanup mechanisms.
* **Testing**: Create comprehensive tests for state management logic, including edge cases and error conditions.
* **Documentation**: Provide clear documentation for state structures and their intended use cases. 