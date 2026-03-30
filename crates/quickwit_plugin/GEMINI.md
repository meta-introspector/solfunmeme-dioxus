# AI Agent Directives for `quickwit_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `quickwit_plugin` crate.

## Core Principles

When working within `quickwit_plugin`, AI agents should prioritize:

1.  **Efficient Data Indexing:** Ensure optimal performance and resource utilization during data indexing operations using Quickwit.
2.  **Reliable Search Functionality:** Provide robust and accurate search capabilities leveraging Quickwit's search engine.
3.  **Scalability:** Design the plugin to scale effectively with increasing data volumes and query loads.

## Operational Guidelines

*   **Configuration:** Handle Quickwit configuration parameters (e.g., schema definition, indexing settings) correctly.
*   **Data Ingestion:** Implement efficient data ingestion pipelines to feed data into Quickwit.
*   **Query Construction:** Construct Quickwit queries accurately to retrieve desired search results.
*   **Error Handling:** Implement comprehensive error handling for all Quickwit interactions.
