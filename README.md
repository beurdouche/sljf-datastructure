# Starlit Jellyfish datastructure implementation

A Rust implementation of a simple Prefix Trie (also known as a Radix Tree), designed for efficient key-value storage with string-based keys. This implementation uses a specific colon-terminated key convention for leaf nodes.

## Overview

This library provides a tree-based data structure that:

-   Efficiently stores and retrieves key-value pairs with shared prefixes.
-   Uses `String` keys and `String` values.
-   Follows a specific logic for splitting and merging nodes based on common prefixes.

## Tree Structure

The implementation supports two node types, defined in the `Node` enum:

-   **`Node::Node { children: HashMap<String, Box<Node>> }`**: A branch node that contains a map to its children. The keys in the map are the edge labels. A special `":"` key is used to store a value at an intermediate node path.
-   **`Node::Leaf { value: String }`**: A leaf node that contains the final value. The key for this leaf is stored as an edge in its parent's `children` map, ending with a `":"`.

## Features

-   **Efficient Storage**: Shared prefixes reduce memory usage.
-   **String-based Keys**: Simple to use with string keys.
-   **Visualizable**: Comes with a `display_tree()` method to print the tree structure to the console for easy debugging.

## Usage

### Basic Operations

```rust
use sljf_datastructure::PrefixTree;

// Create a new tree
let mut tree = PrefixTree::new();

// Insert key-value pairs
tree.insert("foo", "value1".to_string());
tree.insert("foobar", "value2".to_string());
tree.insert("foofoo", "value3".to_string());
tree.insert("bar", "value4".to_string());

// Retrieve values
assert_eq!(tree.get("foo"), Some("value1".to_string()));
assert_eq!(tree.get("foobar"), Some("value2".to_string()));

// Get tree statistics
println!("Tree size: {}", tree.len());
println!("Is empty: {}", tree.is_empty());

// Display the tree structure
tree.display_tree();
```

## API Reference

### Core Types

-   `PrefixTree`: The main tree structure.
-   `Node`: Enum representing `Node` and `Leaf` node types.
-   `Hash`: 32-byte array type for cryptographic hashes (utility, not used in the tree structure itself).

### Main Methods

-   `new()`: Creates an empty `PrefixTree`.
-   `default()`: Creates an empty `PrefixTree`.
-   `insert(key: &str, value: String)`: Inserts or updates a key-value pair.
-   `get(key: &str) -> Option<String>`: Retrieves a value by key.
-   `keys() -> Vec<String>`: Gets all keys in the tree.
-   `len() -> usize`: Gets the number of key-value pairs in the tree.
-   `is_empty() -> bool`: Checks if the tree is empty.
-   `display_tree()`: Prints a visual representation of the tree to the console.

## Hashing Utility

The library includes a `hash_bytes` function that uses **Keccak256** for cryptographic hashing.

```rust
use sljf_datastructure::hash_bytes;

let hash = hash_bytes(b"some data");
println!("Keccak256 hash: {:?}", hash);
```
Note: This hashing function is provided as a utility and is not currently integrated into the `PrefixTree` structure for integrity verification.

## Testing

Run the test suite:

```bash
cargo test
```

The tests cover:
- Basic insertion and retrieval.
- A specific user-provided scenario for node splitting and merging.
- Key collection.

## Dependencies

-   `sha3`: For Keccak256 cryptographic hashing.
-   `serde`: For serialization support of the `Node` enum.

## License

This project is licensed under the MIT OR Apache-2.0 license. (Assuming, please update if incorrect)
