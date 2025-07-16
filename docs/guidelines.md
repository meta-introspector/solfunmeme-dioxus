# Development Guidelines for the Code-Math Manifold

These guidelines are the foundational principles for contributing to the `solfunmeme-dioxus` project. They are designed to ensure our codebase is modular, robust, maintainable, and philosophically aligned with our core mission: exploring the intersection of code, mathematics, and meaning.

## Core Architectural Principles

### 1. **Embrace DRY (Don't Repeat Yourself)**
-   **Principle:** Every piece of knowledge must have a single, unambiguous, authoritative representation within a system. Avoid duplicating code, logic, or data structures.
-   **Rationale:** Duplication leads to maintenance nightmares. A change in one place requires finding and updating all duplicates, which is error-prone and time-consuming.
-   **Do:** Abstract common logic into reusable functions, macros, or centralized services (e.g., the `utils` module in `solfunmeme_tools`). Define core data types in a single, dedicated crate (like `solfunmeme_function_analysis`) and import them where needed.
-   **Don't:** Copy and paste code blocks. Avoid magic strings or numbers; define them as constants in a shared module.

### 2. **Leverage Strong, Semantic Typing**
-   **Principle:** Use Rust's type system to its fullest potential to make illegal states unrepresentable. Create new types to represent distinct concepts, even if their underlying representation is simple.
-   **Rationale:** Strong typing catches errors at compile time, not runtime. It makes code self-documenting and easier to reason about.
-   **Do:** Use `enum`s to represent a fixed set of states (e.g., `ChunkType`). Use the newtype pattern (`struct MyId(String);`) to prevent accidentally mixing up different kinds of IDs.
-   **Don't:** Use primitive types like `String` or `i32` for everything. Avoid passing around loosely structured `serde_json::Value` objects when a concrete `struct` can be defined.

### 3. **Isolate and Insulate External Dependencies**
-   **Principle:** Wrap external libraries and services in dedicated modules or crates. The rest of our application should interact with our abstraction, not directly with the third-party code.
-   **Rationale:** This insulates our core logic from breaking changes in external libraries, makes dependencies swappable, and simplifies testing by allowing us to mock the abstraction layer.
-   **Do:** Create a `quickwit_plugin` or `s3_plugin` that exposes a clean, project-specific API for searching or storage. Use feature flags (`lightweight` vs. `full`) to make heavy dependencies optional.
-   **Don't:** Scatter calls to an external library (e.g., `tantivy`, `reqwest`) throughout the entire codebase.

### 4. **Adhere to the "File=Function=Block=Vibe" Principle**
-   **Principle:** Each file, module, and function should have a single, well-defined responsibility. Keep components small, focused, and composable.
-   **Rationale:** This principle, a cornerstone of our project, dramatically improves modularity, testability, and navigability. It lowers the cognitive load required to understand any single piece of the system.
-   **Do:** Create a new file for each major struct and its `impl` block. Group related, small functions into a module that represents a single "basic block" of functionality.
-   **Don't:** Create monolithic files with thousands of lines of code containing unrelated logic.

### 5. **Practice Minimalist Imports**
-   **Principle:** Only `use` what you need. Be explicit about imports and avoid wildcard (`*`) imports.
-   **Rationale:** This prevents namespace pollution, makes dependencies clear at a glance, and avoids ambiguity. It's immediately obvious where each type or function comes from.
-   **Do:** Write `use std::collections::HashMap;`.
-   **Don't:** Write `use std::collections::*;`.

## Implementation & Workflow

### 6. **Isolate Tests in Fixtures**
-   **Principle:** Test code and test data (fixtures) should live separately from implementation code.
-   **Rationale:** This keeps the implementation modules clean and focused on their primary logic. It also allows for the reuse of complex fixtures across multiple tests.
-   **Do:** Place test-specific files, like large code snippets or sample data, in a `tests/fixtures/` directory. Load them using `include_str!` or file I/O within your tests.
-   **Don't:** Embed large, multi-line string literals for test data directly inside your test functions.

### 7. **Use Monadic Structures for I/O and Control Flow**
-   **Principle:** Embrace Rust's monadic types, `Result<T, E>` and `Option<T>`, to handle I/O, potential failures, and optionality in a clear, explicit, and functional way.
-   **Rationale:** This is the idiomatic Rust approach for error handling and managing optional values. It forces the caller to handle all possible outcomes, leading to more robust and predictable code.
-   **Do:** Use the `?` operator to propagate errors cleanly up the call stack. Use methods like `map`, `and_then`, and `unwrap_or_else` to compose operations on `Result` and `Option`.
-   **Don't:** Use `.unwrap()` or `.expect()` in application logic; reserve them for tests or situations where a panic is the only logical outcome of a variant being present. Avoid returning sentinel values like `-1` or empty strings to indicate failure.

### 8. **Design for Composable Workflows**
-   **Principle:** Every major piece of functionality should be designed as a "workflow" — a self-contained, composable unit that can be chained with others.
-   **Rationale:** This allows us to build complex data processing pipelines from simple, reusable, and independently testable parts. It is central to the project's goal of creating emergent behavior from simple rules.
-   **Do:** Use channels, iterators, or a dedicated workflow manager to pass data between components. Ensure each workflow has clearly defined inputs and outputs.
-   **Don't:** Create tightly coupled components that make assumptions about the internal state of other components.

## Documentation & Philosophy

### 9. **Maintain a Clean Namespace**
-   **Principle:** Names of functions, variables, modules, and crates should be clear, descriptive, and unambiguous.
-   **Rationale:** Clear naming is a form of documentation. It reduces the need for explanatory comments and makes the code easier to read and understand.
-   **Do:** Choose names that reflect the *semantic meaning* of the component, like `ChunkProcessor` or `extract_speaker_and_content`.
-   **Don't:** Use short, cryptic names like `proc`, `x`, or `data_handler`.

### 10. **Elevate Documentation**
-   **Principle:** Documentation is a first-class citizen. It should be written for clarity and discoverability. Important, high-level documentation should be "pushed upwards" in the directory structure or linked from a central place like the main `README.md`.
-   **Rationale:** Good documentation is useless if no one can find it. A clear hierarchy ensures that developers can start with a high-level overview and progressively drill down into details.
-   **Do:** Add a new `doc/my_feature.md` file and link to it from the main `README.md`. Add `#[doc = "..."]` comments to public functions and structs.
-   **Don't:** Bury critical design documents deep inside a crate's source directory. Let documentation rot; keep it updated as the code evolves.

## Additional Suggested Guidelines

### 11. **Treat Code as a Mathematical Object**
-   **Principle:** Always consider the underlying mathematical structure of the code you are writing. Think in terms of graphs, vectors, sets, and transformations.
-   **Rationale:** This is the core philosophy of the Code-Math Manifold. It encourages a deeper, more abstract understanding of software and unlocks novel ways to analyze, visualize, and generate code.
-   **Do:** When parsing code, think of the AST as a tree or graph to be traversed. When creating embeddings, consider the geometric properties of the resulting vector space.
-   **Don't:** View code as just a sequence of imperative instructions.

### 12. **Favor Immutability and Pure Functions**
-   **Principle:** Declare variables as immutable (`let`) by default. Prefer pure functions (functions with no side effects) wherever possible.
-   **Rationale:** Immutability and purity make code easier to reason about, test, and parallelize. They reduce the chance of spooky-action-at-a-distance bugs.
-   **Do:** Use `let mut` only when mutation is necessary. Prefer functions that take data as input and return new data as output, rather than modifying data in-place.
-   **Don't:** Make everything mutable by default. Write functions that have hidden dependencies on or modify global state.

## Development Workflow & Process

### 13. **Commit Incrementally and with Intent**
-   **Principle:** Make small, atomic commits. Before executing any significant change, write your intent to a file (e.g., `intent.md`) and commit it. This creates a clear, auditable trail of your thought process.
-   **Rationale:** Atomic commits are easier to understand, review, and revert if necessary. Committing your intent first ensures that your plan is recorded, which is invaluable for asynchronous collaboration and for recovery if you are interrupted.
-   **Do:** A commit should represent a single logical change. Before a large refactor, create a file outlining the goal, the proposed changes, and the affected files, then commit it with a message like "feat: Plan refactoring of the chunking processor."
-   **Don't:** Make a single, massive commit with unrelated changes. Start making changes without a clear, documented plan.

### 14. **Create, Then Destroy: Safe Refactoring**
-   **Principle:** When refactoring or replacing a component, write the new implementation first. Once the new component is integrated and tested, you can safely delete the old one.
-   **Rationale:** This practice ensures the system remains in a working state at all times, minimizing disruption. It allows for A/B testing between the old and new implementations and makes it trivial to roll back if the new version has issues.
-   **Do:** Create `chunk_processor_v2.rs`, integrate it, and verify its functionality. Once confirmed, delete `chunk_processor.rs`.
-   **Don't:** Start by deleting the old code, leaving the application in a broken state until you finish the new implementation.

### 15. **Document as You Go**
-   **Principle:** Update documentation in the same commit as the code change that necessitates it.
-   **Rationale:** This ensures that documentation never becomes stale or out of sync with the implementation. It treats documentation as an integral part of the development process, not an afterthought.
-   **Do:** When adding a new feature to the `chunk_processor`, update `doc/chat_processing.md` to reflect the new functionality in the same commit.
-   **Don't:** Let documentation changes pile up for a separate "docs" commit.

### 16. **Embrace Interruption-Driven Development**
-   **Principle:** Assume you can be interrupted at any time. Your workflow should be resilient to context switching.
-   **Rationale:** This principle is a practical reality. By committing your intent and making small, atomic changes, you ensure that you or another developer can easily pick up where you left off, even after a long interruption.
-   **Do:** Keep your work in a state that can be quickly understood. Rely on your `intent.md` file and `git log` to restore context.
-   **Don't:** Keep large, uncommitted changes on your local machine for extended periods. Rely on your memory to track your progress.