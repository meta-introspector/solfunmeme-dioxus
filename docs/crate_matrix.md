## 🟢 OODA Dashboard: Crate Matrix (Observer Phase)

We are currently in the **Observer** phase of the OODA (Observe, Orient, Decide, Act) loop: indexing, introspecting, and mapping the system.

| Crate Name                  | Emoji  | Function / OODA Phase         | Description                                                      |
|-----------------------------|--------|-------------------------------|------------------------------------------------------------------|
| solfunmeme_tools            | 🧰     | Observer (Utility)            | General utilities, logging, error handling, string/data helpers   |
| solfunmeme_indexer          | 🗂️     | Observer (Indexing)           | Indexes code chunks, builds search indices, generates reports     |
| solfunmeme_core_logic       | 🧠     | Orient (Core Logic)           | Core business/data logic, main algorithms                        |
| solfunmeme_function_analysis| 🔬     | Observer (Analysis)           | Analyzes and extracts functions from code                        |
| solfunmeme_input_fs         | 📂     | Observer (Input)              | Handles file system input, code chunking                         |
| solfunmeme_search_tantivy   | 🔎     | Observer (Search)             | Full-text search and indexing with Tantivy                       |
| solfunmeme_state            | 🗃️     | Orient (State)                | Project state management                                         |
| solfunmeme_playground       | 🧪     | Orient (Experimentation)      | Interactive playground for testing and prototyping               |
| solfunmeme_models           | 🏗️     | Orient (Models)               | Data models and types                                            |
| solfunmeme_extractor_system | 🏷️     | Observer (Extraction)         | System for extracting and labeling code/data                     |
| solfunmeme_embedding        | 🧬     | Orient (Embedding)            | Embedding and vectorization tools                                |
| solfunmeme_clifford         | 🧮     | Orient (Math)                 | Clifford algebra and mathematical operations                     |
| solfunmeme_app              | 🖥️     | Act (App/UI)                  | Main application and user interface                              |
| solfunmeme_wallet_integration| 💳    | Act (Blockchain)              | Solana wallet and blockchain integration                         |
| solfunmeme_solana_data      | 🔗     | Act (Blockchain Data)         | On-chain data and Solana-specific logic                          |
| solfunmeme_views            | 🪟     | Act (UI Components)           | UI components and views                                          |
| solfunmeme_tantivy_report   | 📊     | Orient (Reporting)            | Reporting and analytics from search/index                        |
| solfunmeme_broken_tantivy   | 🛠️     | Orient (Experimental)         | Experimental/legacy Tantivy integration                          |
| task_manager                | ✅     | Orient (Task Management)      | Task and workflow management                                     |
| workflow_manager            | 🔄     | Orient (Workflow)             | Workflow orchestration                                           |
| ...                         | ...    | ...                           | ...                                                              |

> **Legend:**
> - 🟢 Observer: Indexing, introspection, analysis
> - 🟡 Orient: Data modeling, logic, experimentation
> - 🟠 Decide: (not yet implemented)
> - 🔴 Act: UI, blockchain, user-facing actions

---

For a full semantic index and glossary, see [crates/README.md](crates/README.md) and [founding_documents/GEMINI.md](founding_documents/GEMINI.md).

This dashboard will be updated as the project evolves and new crates or features are added.

**License:** AGPL-3.0