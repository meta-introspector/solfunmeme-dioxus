## Task Management for AI Agents

As an AI agent contributing to `solfunmeme-dioxus`, you can interact with and manage project tasks using the `task_manager` CLI tool. This tool helps track progress, dependencies, and overall project state.

### Key Commands:

*   **`cargo run -p task_manager -- list`**: Lists all currently defined tasks, showing their ID, content, status, and dependencies.
*   **`cargo run -p task_manager -- show <TASK_ID>`**: Displays detailed information about a specific task.
*   **`cargo run -p task_manager -- update-status <TASK_ID> <NEW_STATUS>`**: Updates the status of a task (e.g., `pending`, `in-progress`, `completed`, `blocked`).
*   **`cargo run -p task_manager -- add <TASK_ID> <CONTENT> --dependencies <DEP1> <DEP2> ...`**: Adds a new task. Dependencies are optional.
*   **`cargo run -p task_manager -- InitIndexingTasks`**: Initializes a predefined set of tasks related to codebase indexing and reporting. This command will create JSON files for these tasks in `task_manager/states/gemini/`.

### Workflow for Indexing and Reporting Tasks:

1.  **Initialize Tasks:** When starting a new session or after significant changes to the codebase analysis pipeline, run `cargo run -p task_manager -- InitIndexingTasks` to ensure all relevant indexing and reporting tasks are registered.
2.  **List Tasks:** Use `cargo run -p task_manager -- list` to see the current state of all tasks. Pay attention to task statuses and dependencies.
3.  **Execute Tasks:** Based on the task list, execute the necessary commands. For example:
    *   To index the codebase: `cargo run -p solfunmeme_indexer --bin main -- index . ./tmp/tantivy_index`
    *   To generate a top 10 emoji report: `cargo run -p solfunmeme_indexer --bin main -- report ./tmp/tantivy_index --report-type emoji`
4.  **Update Task Status:** After completing a task, update its status using `cargo run -p task_manager -- update-status <TASK_ID> completed`. If a task is blocked, update its status accordingly.
5.  **Monitor Progress:** Regularly list tasks to monitor overall progress and identify the next steps.