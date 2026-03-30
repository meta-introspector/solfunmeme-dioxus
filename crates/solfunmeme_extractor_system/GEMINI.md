# AI Agent Directives for `solfunmeme_extractor_system`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_extractor_system` crate.

## Core Principles

When working within `solfunmeme_extractor_system`, AI agents should prioritize:

1. **System Safety**: Ensure all system-level operations are safe and handle errors gracefully, especially when dealing with clipboard and file system operations.
2. **Cross-Platform Compatibility**: Design functionality to work across different operating systems and environments.
3. **Performance**: Optimize extraction operations for efficiency, especially when processing large files or datasets.
4. **Error Handling**: Provide comprehensive error handling and meaningful error messages for debugging.

## Operational Guidelines

* **Clipboard Operations**: When implementing clipboard functionality, ensure proper permission handling and fallback mechanisms for different platforms.
* **File Processing**: Implement robust file processing with proper encoding detection and error recovery.
* **Code Testing**: Provide comprehensive validation for code snippets, including syntax checking and basic semantic analysis.
* **Resource Management**: Ensure proper cleanup of system resources and handle memory efficiently.
* **Logging**: Implement appropriate logging for debugging and monitoring extraction operations. 