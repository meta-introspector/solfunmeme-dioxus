# gitaccount

A pure Rust type library for representing Git-like, content-addressed, on-chain repositories, branches, and objects. Designed for use in Solana programs, Dioxus apps, and other Rust projects.

## Architecture: A Hybrid L1/L2 Model

This library is designed to operate in a hybrid environment that combines a high-throughput custom validator (L2) with the security of the Solana mainnet (L1).

-   **Layer 2 (Custom Validator / Subnet):** The primary environment where the full git history is stored. All `Repo`, `Branch`, `Commit`, `Tree`, and `Blob` objects exist as individual, content-addressed accounts on a custom validator. This allows for low-cost storage and high-throughput operations, making on-chain git operations practical. Blobs and other objects can be "rolled up" or aggregated efficiently within this subnet.

-   **Layer 1 (Solana Mainnet):** The trust and settlement layer. Instead of storing the entire repository on the expensive mainnet, we only post a Zero-Knowledge Proof (ZKP) and the final state hash of a given release. This ZKP cryptographically proves the integrity of the release and its history on the L2 subnet, providing ultimate security and verifiability without incurring high storage costs.

This hybrid model provides the best of both worlds: the performance and low cost of a dedicated subnet with the trustless security and decentralization of the Solana mainnet.

## C4 Context Diagram (PlantUML)

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

Person(user, "User", "Developer or Agent interacting with the system")
System_Boundary(gitaccount, "On-chain GitAccount System") {
  Container(repo, "Repo Account", "Solana Account", "Stores repo metadata and root tree hash")
  Container(branch, "Branch Account", "Solana Account", "Represents a branch, points to head commit")
  Container(object, "Git Object Account", "Solana Account", "Blob, Tree, or Commit, addressed by hash")
}
System(solana, "Solana Blockchain", "Stores all accounts and provides program execution")
System(dioxus, "Dioxus App", "Frontend or agent UI")

Rel(user, dioxus, "Uses")
Rel(dioxus, gitaccount, "Interacts with")
Rel(gitaccount, solana, "Stores data on")
Rel(repo, branch, "Has many")
Rel(branch, object, "Points to head commit (object)")
Rel(repo, object, "Root tree, links to objects")
@enduml
```

## Design Principles

- **Content-addressed:** All objects are addressed by their hash (SHA-256 or compatible with Solana pubkeys).
- **Minimal:** Only core types and relationships, no serialization or protocol logic included by default.
- **Extensible:** Can be extended with serialization, Solana-specific logic, or UI integration as needed.

## Core Types

- `Repo`: Metadata for a repository (id, name, description, owner, root tree, etc.)
- `Branch`: Represents a branch in a repo (id, repo, name, head commit, etc.)
- `GitObject`: Enum for Blob, Tree, Commit
- `Blob`: File contents
- `Tree`: Directory structure (entries)
- `Commit`: Commit metadata (tree, parents, author, message, timestamp)

## Example Use Cases

- **On-chain Git:** Store and traverse full Git repositories on Solana or other blockchains.
- **Decentralized build systems:** Each build artifact or stage is a derived object/account.
- **Collaborative editing:** Branches and forks as token accounts or separate branches.
- **Provenance and reproducibility:** Every artifact is cryptographically linked to its source and build process.

## Next Steps

- Add serialization support (`serde`, `borsh`, etc.)
- Implement helper functions for object creation, hashing, and linking
- Integrate with Solana programs or Dioxus apps
- Explore on-chain build, merge, and collaboration workflows

---

*This crate is a work in progress and a foundation for decentralized, content-addressed code and data systems.* 