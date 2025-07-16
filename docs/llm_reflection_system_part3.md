# LLM Reflection System: Intent and Architecture - Part 3

## 3. Decoupling the "Bid" from the "Ask"

A key architectural decision was to decouple the "bid" (the vectorized representation of the code that needs reflection) from the "ask" (the cost and execution of the LLM model). This is achieved by:

*   **`prepare_sources` as the Vectorizer:** The `prepare_sources` tool is now solely responsible for generating `CodeChunk`s and their associated BERT embeddings. It outputs these vectorized chunks, effectively creating the "bid" data.
*   **Market Maker's Role in Cost Management:** The `solfunmeme_market_maker` is responsible for understanding the "ask" (LLM provider costs and constraints) and making informed decisions about which provider to use for each reflection task. This centralizes cost management and allows for flexible switching between providers without re-processing the code chunks.

## 4. Communication Protocol

Inter-process communication between the `llm_planner_cli`, `solfunmeme_market_maker`, and the external LLM providers is standardized using **JSON over standard I/O (stdin/stdout)**. This provides a language-agnostic and flexible interface, allowing LLM providers to be implemented in any language.
