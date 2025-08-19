# AI Agent Directives for `s3_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `s3_plugin` crate.

## Core Principles

When working within `s3_plugin`, AI agents should prioritize:

1.  **Secure S3 Interactions:** Ensure all interactions with AWS S3 are secure, utilizing proper authentication and authorization mechanisms.
2.  **Data Integrity:** Maintain data integrity during uploads and downloads to and from S3 buckets.
3.  **Efficiency:** Optimize S3 operations for performance, especially when dealing with large files or frequent transfers.

## Operational Guidelines

*   **Credential Management:** Handle AWS credentials securely, avoiding hardcoding or exposure.
*   **Error Handling:** Implement robust error handling for all S3 operations, providing clear feedback in case of failures.
*   **Region and Endpoint Configuration:** Ensure correct configuration of AWS regions and S3 endpoints.