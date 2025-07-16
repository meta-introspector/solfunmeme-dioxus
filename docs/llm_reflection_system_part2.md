# LLM Reflection System: Intent and Architecture - Part 2

## 2. Architectural Overview: A Distributed Bid/Ask System (Continued)

### Key Roles (Continued):

*   **The Market Maker (`solfunmeme_market_maker`):** This is the central orchestrator, acting as the "market maker" between the buyers and the LLM providers. Its responsibilities include:
    *   **Consuming Needs:** It receives structured LLM reflection task groups from the `llm_planner_cli` (via `stdin`).
    *   **Provider Matching:** It maintains a configuration of available LLM providers, including their `UsageVector` (cost per token, rate limits, available credits, etc.).
    *   **Optimal Provider Selection:** For each incoming task group, it dynamically selects the most suitable LLM provider and account based on predefined criteria (e.g., prioritize accounts with available credits, then lowest cost, then highest rate limit).
    *   **Task Dispatching:** It dispatches the reflection tasks to the selected LLM provider by spawning the provider's executable as a subprocess and communicating via JSON over standard I/O.
    *   **Response Handling:** It receives the reflection results from the LLM provider and can log them, update usage metrics, or route them to further processing stages.

*   **The LLM Providers (External Binaries, e.g., `llm_echo_provider`):** These are standalone executables that represent the "sellers" of LLM services. Each provider is responsible for:
    *   **Consuming Tasks:** Reading an `LlmTaskGroup` (JSON) from its `stdin`.
    *   **Performing Reflection:** Executing the actual LLM call or reflection logic (e.g., calling a remote API, running a local model).
    *   **Producing Results:** Writing the reflection results (JSON) to its `stdout`.
    *   **Echo Mode:** For testing and development, providers can operate in an "echo mode" (like `llm_echo_provider`), simply returning the input to demonstrate the communication pipeline.
