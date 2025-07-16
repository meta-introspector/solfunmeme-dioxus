# `solana_integration_lib`

This crate provides data models and functions for integrating with the Solana blockchain, specifically for representing code-related entities and managing buy orders on-chain.

## Key Data Structures

### Structs:

*   **`Ontology`**: Represents an ontology on the Solana blockchain, defining its ID, classes, properties, creator, and timestamp.
*   **`CodeFile`**: Stores information about a code file, including its name, path, references to functions and buy orders, associated ontology, creator, and timestamp.
*   **`Term`**: Represents a term with its text, description, references to emojis and functions, associated buy order, ontology, creator, and timestamp.
*   **`BuyOrder`**: Defines a buy order with a unique ID, source code chunk, vectorized result, reference to the instance, status, associated ontology, creator, creator, and timestamp.

### Enums:

*   **`BuyOrderStatus`**: Defines the possible states of a buy order: `Open`, `OfferReceived`, and `Fulfilled`.