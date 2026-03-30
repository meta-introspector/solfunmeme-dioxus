# AI Agent Directives for `zip_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `zip_plugin` crate.

## Core Principles

When working within `zip_plugin`, AI agents should prioritize:

1.  **Data Compression/Decompression Integrity:** Ensure that all compression and decompression operations maintain data integrity and do not introduce corruption.
2.  **Efficiency:** Optimize compression and decompression processes for speed and resource usage.
3.  **Compatibility:** Ensure compatibility with common ZIP file formats and standards.

## Operational Guidelines

*   **File Handling:** Handle file I/O operations carefully to prevent data loss or corruption during zipping/unzipping.
*   **Error Handling:** Implement robust error handling for all compression/decompression operations, providing clear feedback in case of failures.
*   **Security:** Be mindful of potential security vulnerabilities related to handling compressed archives (e.g., zip bombs, path traversal).