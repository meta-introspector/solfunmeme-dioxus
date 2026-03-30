# `git_plugin`

This crate provides a wrapper around Git functionalities, enabling programmatic interaction with Git repositories within the Solfunmeme ecosystem.

## Purpose

It allows the application to perform version control operations such as cloning repositories, pulling updates, committing changes, and pushing to remotes, facilitating code management and data synchronization.

## Core Functionalities

-   **Clone Repository**: Clone a Git repository from a given URL to a local path.
-   **Open Repository**: Open an existing local Git repository.
-   **Pull Repository**: Fetch and merge changes from a remote repository.
-   **Commit Changes**: Stage and commit changes to the local repository.
-   **Push Changes**: Push committed changes to a remote repository.

## Usage (Conceptual)

```rust
use git_plugin::{clone_repository, open_repository, pull_repository, commit_changes, push_changes};
use std::path::Path;

fn main() {
    let repo_url = "https://github.com/example/repo.git";
    let local_path = Path::new("./my_repo");

    // Example: Clone a repository
    // let repo = clone_repository(repo_url, local_path).expect("Failed to clone");

    // Example: Open an existing repository
    // let repo = open_repository(local_path).expect("Failed to open");

    // Example: Pull changes
    // pull_repository(&repo).expect("Failed to pull");

    // Example: Commit changes
    // commit_changes(&repo, "My commit message").expect("Failed to commit");

    // Example: Push changes
    // push_changes(&repo).expect("Failed to push");
}
```
