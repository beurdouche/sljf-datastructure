# Merkle Patricia Tree Library

A Rust implementation of a **Merkle Patricia Tree** (also known as a Radix Tree or Compressed Trie), combining the features of Patricia Trees and Merkle Trees for efficient key-value storage with cryptographic integrity verification.

## Overview

This library provides a tree-based data structure that:

- Efficiently stores and retrieves key-value pairs with shared prefixes
- Maintains cryptographic integrity through hash chains
- Supports arbitrary byte keys and values
- Uses compressed paths to minimize storage overhead
- Enables fast prefix-based lookups

## Tree Structure

The implementation supports four node types:

- **Leaf Node**: Contains the final key suffix and value
- **Extension Node**: Represents shared key prefixes pointing to child nodes
- **Branch Node**: Has up to 16 children (for hexadecimal digits) and optionally a value
- **Empty Node**: Represents absence of data

```
       Root
        |
    Extension("foo")
       / \
   Leaf(bar) Branch
   "value1"   / | \
            /  |  \
     Leaf("")  |  Leaf(foo)
    "value2"   |  "value3"
               |
           Leaf(bar)
           "value4"
```

## Features

- **Efficient Storage**: Compressed paths reduce memory usage for sparse key spaces
- **Fast Retrieval**: O(k) lookup time where k is the key length
- **Cryptographic Integrity**: Each node is identified by its hash, enabling verification
- **Flexible Keys**: Supports arbitrary byte sequences as keys
- **Thread-Safe**: The core data structure can be safely shared across threads

## Usage

### Basic Operations

```rust
use sljf_datastructure::MerklePatriciaTree;

// Create a new tree
let mut tree = MerklePatriciaTree::new();

// Insert key-value pairs
tree.insert(b"foo", b"value1".to_vec()).unwrap();
tree.insert(b"foobar", b"value2".to_vec()).unwrap();
tree.insert(b"foofoo", b"value3".to_vec()).unwrap();
tree.insert(b"bar", b"value4".to_vec()).unwrap();

// Retrieve values
assert_eq!(tree.get(b"foo"), Some(b"value1".to_vec()));
assert_eq!(tree.get(b"foobar"), Some(b"value2".to_vec()));

// Check existence
assert!(tree.contains_key(b"foo"));
assert!(!tree.contains_key(b"nonexistent"));

// Get tree statistics
println!("Tree size: {}", tree.len());
println!("Is empty: {}", tree.is_empty());
```

### Hash Verification

```rust
// Get the root hash for integrity verification
if let Some(root_hash) = tree.root_hash() {
    println!("Root hash: {:?}", root_hash);
}

// Verify the entire tree's integrity
assert!(tree.verify_integrity());
```

### Key Enumeration

```rust
// Get all keys in the tree
let keys = tree.keys();
for key in keys {
    println!("Key: {:?}", String::from_utf8_lossy(&key));
}
```

## API Reference

### Core Types

- `MerklePatriciaTree`: The main tree structure
- `Node`: Enum representing different node types
- `Hash`: 32-byte array type for cryptographic hashes

### Main Methods

- `new()`: Create an empty tree
- `insert(key, value)`: Insert or update a key-value pair
- `get(key)`: Retrieve a value by key
- `contains_key(key)`: Check if a key exists
- `keys()`: Get all keys in the tree
- `len()`: Get the number of key-value pairs
- `is_empty()`: Check if the tree is empty
- `root_hash()`: Get the root hash for verification
- `verify_integrity()`: Verify the cryptographic integrity of the entire tree

## Implementation Details

### Key Encoding

Keys are converted to **nibbles** (4-bit values) for efficient traversal:

- Each byte becomes two nibbles
- Enables hexadecimal branching in branch nodes
- Allows for optimal prefix compression

### Hashing

The library uses **Keccak256** for cryptographic hashing:

- Each node is identified by the hash of its serialized content
- Provides strong integrity guarantees
- Compatible with Ethereum and other blockchain systems

### Memory Layout

Nodes are stored in a `HashMap<Hash, Node>` structure:

- Efficient O(1) node access by hash
- Enables structural sharing and deduplication
- Supports persistent data structures

## Performance Characteristics

- **Space Complexity**: O(n) where n is the total size of all keys and values
- **Time Complexity**:
  - Insert: O(k) where k is the key length
  - Lookup: O(k) where k is the key length
  - Delete: O(k) where k is the key length (when implemented)

## Use Cases

This data structure is particularly well-suited for:

- **Blockchain State Trees**: Ethereum uses Merkle Patricia Trees for state storage
- **Version Control Systems**: Efficient storage of file hierarchies
- **Prefix-based Routing**: Network routing tables with shared prefixes
- **Autocomplete Systems**: Fast prefix matching for suggestions
- **Configuration Management**: Hierarchical configuration storage

## Testing

Run the test suite:

```bash
cargo test
```

The tests cover:

- Basic insertion and retrieval
- Tree integrity verification
- Hash consistency
- Edge cases and error conditions
- Performance scenarios

## Dependencies

- `sha3`: Keccak256 cryptographic hashing
- `serde`: Serialization support
- `bincode`: Binary serialization format
- `hex`: Hexadecimal encoding utilities

## License

This project is licensed under the MIT OR Apache-2.0 license.

## Contributing

Contributions are welcome! Please ensure that:

- All tests pass
- Code follows the existing style
- New features include appropriate tests
- Documentation is updated for public APIs

## Future Enhancements

Planned improvements include:

- [ ] Delete operation support
- [ ] Iterator implementations
- [ ] Persistence layer integration
- [ ] Parallel tree construction
- [ ] Memory usage optimizations
- [ ] Merkle proof generation and verification
