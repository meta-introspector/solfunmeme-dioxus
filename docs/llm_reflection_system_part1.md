# LLM Reflection System: Intent and Architecture - Part 1

## 1. Intent: Leveraging External LLMs for Code Reflection

The primary goal of this system is to enable the project to leverage external Large Language Models (LLMs) for deep reflection and analysis of our codebase. This reflection can encompass various tasks, such as:

*   **Code Understanding:** Generating summaries, explanations, or documentation for complex code sections.
*   **Refactoring Suggestions:** Identifying areas for improvement, potential bugs, or architectural inconsistencies.
*   **Semantic Analysis:** Extracting higher-level meaning and relationships from code beyond what traditional static analysis can provide.
*   **Knowledge Graph Population:** Enriching our internal knowledge graphs with LLM-derived insights.

Given the varying costs, capabilities, and rate limits of different LLM providers (e.g., Groq, Grok, Gemini, OpenAI, OpenRouter), a distributed and flexible architecture is crucial to optimize resource utilization and ensure efficient task execution.

## 2. Architectural Overview: A Distributed Bid/Ask System

We've designed a distributed system that separates the generation of reflection tasks ("bids") from the execution of those tasks by LLMs ("asks"). This allows for dynamic provider selection and cost optimization.

### Key Roles:

*   **The Buyer (`llm_planner_cli`):** This component acts as the "buyer" of LLM reflection services. Its primary responsibilities are:
    *   **Code Chunking and Embedding:** It utilizes the `prepare_sources` tool to process the codebase, extract meaningful `CodeChunk`s, and generate high-dimensional BERT embeddings for each chunk.
    *   **Task Grouping:** It groups semantically similar code chunks together using cosine similarity on their embeddings. Each group represents a coherent unit of work for an LLM.
    *   **Needs Generation:** It outputs these grouped tasks (needs) in a structured format (JSON) to be consumed by the Market Maker.
    *   **Initial Provider Selection (Optional):** While the Market Maker makes the final decision, the planner can provide an initial suggestion or filter based on high-level criteria.
