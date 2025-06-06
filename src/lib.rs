//! # Simple Prefix Trie Implementation
//!
//! This crate provides a simple prefix trie (tree) data structure that follows the exact
//! specification provided by the user, using string keys with colon terminators.

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/// A hash type used for node identification and integrity verification
pub type Hash = [u8; 32];

/// Convert bytes to a hash using Keccak256
pub fn hash_bytes(data: &[u8]) -> Hash {
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Node types in the Prefix Tree.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Node {
    /// Leaf node containing a value. The key for this leaf is the edge in the parent's map.
    Leaf { value: String },
    /// A branch node that has children. It can also have a value, stored in a special ":" child.
    Node {
        children: HashMap<String, Box<Node>>,
    },
}

/// The main Prefix Tree structure.
#[derive(Debug, Clone)]
pub struct PrefixTree {
    root: Box<Node>,
}

impl Default for PrefixTree {
    fn default() -> Self {
        Self::new()
    }
}

impl PrefixTree {
    /// Create a new empty Prefix Tree with a ROOT node.
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::Node {
                children: HashMap::new(),
            }),
        }
    }

    /// Find the length of the common prefix between two strings.
    fn common_prefix_length(a: &str, b: &str) -> usize {
        a.chars()
            .zip(b.chars())
            .take_while(|(ca, cb)| ca == cb)
            .count()
    }

    /// Insert a key-value pair into the tree.
    pub fn insert(&mut self, key: &str, value: String) {
        Self::insert_recursive(&mut self.root, key, value);
    }

    fn insert_recursive(node: &mut Node, key: &str, value: String) {
        let key_with_colon = format!("{}:", key);

        if let Node::Node { children } = node {
            // Case 1: The key is a prefix of an existing child's key.
            // e.g., inserting "foo" when "foobar" exists.
            let mut prefix_child_key: Option<String> = None;
            if !key.is_empty() {
                for child_key in children.keys() {
                    if child_key.starts_with(key) && child_key.len() > key.len() {
                        prefix_child_key = Some(child_key.clone());
                        break;
                    }
                }
            }

            if let Some(child_key) = prefix_child_key {
                let existing_child = children.remove(&child_key).unwrap();
                let suffix = &child_key[key.len()..];

                let mut new_node_children = HashMap::new();
                new_node_children.insert(":".to_string(), Box::new(Node::Leaf { value }));
                new_node_children.insert(suffix.to_string(), existing_child);

                children.insert(
                    key.to_string(),
                    Box::new(Node::Node {
                        children: new_node_children,
                    }),
                );
                return;
            }

            // Case 2: An existing child's key is a prefix of the new key.
            // e.g., inserting "foobar" when "foo" node exists.
            let mut child_prefix_key: Option<String> = None;
            for child_key in children.keys() {
                if !child_key.ends_with(':') && key.starts_with(child_key) {
                    child_prefix_key = Some(child_key.clone());
                    break;
                }
            }
            if let Some(child_key) = child_prefix_key {
                let child_node = children.get_mut(&child_key).unwrap();
                let suffix = &key[child_key.len()..];
                Self::insert_recursive(child_node, suffix, value);
                return;
            }

            // Case 3: Partial prefix match - needs splitting.
            // e.g., inserting "foofoo" when "foobar" exists.
            let mut best_match_key: Option<String> = None;
            let mut max_common_len = 0;

            for child_key in children.keys() {
                let compare_key = if child_key.ends_with(':') {
                    &child_key[..child_key.len() - 1]
                } else {
                    child_key
                };
                let common_len = Self::common_prefix_length(key, compare_key);
                if common_len > max_common_len {
                    max_common_len = common_len;
                    best_match_key = Some(child_key.clone());
                }
            }

            if let Some(child_key) = best_match_key {
                if max_common_len > 0 && max_common_len < key.len() {
                    let child_compare_key = if child_key.ends_with(':') {
                        &child_key[..child_key.len() - 1]
                    } else {
                        &child_key
                    };
                    if max_common_len < child_compare_key.len() {
                        let mut existing_child = children.remove(&child_key).unwrap();
                        let shared_prefix = &key[..max_common_len];

                        let new_suffix = &key[max_common_len..];
                        let existing_suffix = &child_key[max_common_len..];

                        let mut new_children = HashMap::new();
                        new_children
                            .insert(format!("{}:", new_suffix), Box::new(Node::Leaf { value }));
                        new_children.insert(existing_suffix.to_string(), existing_child);

                        children.insert(
                            shared_prefix.to_string(),
                            Box::new(Node::Node {
                                children: new_children,
                            }),
                        );
                        return;
                    }
                }
            }

            // Case 4: Exact match or no prefix relationship.
            if let Some(child) = children.get_mut(&key_with_colon) {
                if let Node::Leaf { value: old_value } = child.as_mut() {
                    *old_value = value;
                    return;
                }
            }
            children.insert(key_with_colon, Box::new(Node::Leaf { value }));
        }
    }

    /// Get a value by key from the tree.
    pub fn get(&self, key: &str) -> Option<String> {
        Self::get_recursive(&self.root, key)
    }

    fn get_recursive(node: &Node, key: &str) -> Option<String> {
        if let Node::Node { children } = node {
            // Check for a node value first (a ":" leaf)
            if key.is_empty() {
                if let Some(value_node) = children.get(":") {
                    if let Node::Leaf { value } = &**value_node {
                        return Some(value.clone());
                    }
                }
            }

            // Check for direct leaf match.
            let key_with_colon = format!("{}:", key);
            if let Some(leaf_node) = children.get(&key_with_colon) {
                if let Node::Leaf { value } = &**leaf_node {
                    return Some(value.clone());
                }
            }

            // Check for intermediate node.
            for (child_key, child_node) in children {
                if !child_key.ends_with(':') && key.starts_with(child_key) {
                    let suffix = &key[child_key.len()..];
                    return Self::get_recursive(child_node, suffix);
                }
            }
        }
        None
    }

    /// Get all keys in the tree.
    pub fn keys(&self) -> Vec<String> {
        let mut keys = Vec::new();
        Self::collect_keys_recursive(&self.root, "", &mut keys);
        keys
    }

    fn collect_keys_recursive(node: &Node, prefix: &str, keys: &mut Vec<String>) {
        if let Node::Node { children } = node {
            for (child_key, child_node) in children {
                let new_prefix = format!("{}{}", prefix, child_key);
                if child_key == ":" {
                    keys.push(prefix.to_string());
                } else if child_key.ends_with(':') {
                    keys.push(new_prefix[..new_prefix.len() - 1].to_string());
                } else {
                    Self::collect_keys_recursive(child_node, &new_prefix, keys);
                }
            }
        }
    }

    /// Get the number of key-value pairs in the tree.
    pub fn len(&self) -> usize {
        self.keys().len()
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        if let Node::Node { children } = &*self.root {
            return children.is_empty();
        }
        false
    }

    /// Display the tree structure graphically in the console.
    pub fn display_tree(&self) {
        println!("Prefix Tree Structure:");
        println!("=====================");
        println!("ROOT (branch node, no value)");
        Self::display_node_recursive(&self.root, "   ");
        println!("=====================");
    }

    fn display_node_recursive(node: &Node, prefix: &str) {
        if let Node::Node { children } = node {
            let mut entries: Vec<_> = children.iter().collect();
            entries.sort_by(|a, b| a.0.cmp(b.0));

            for (i, (key, child)) in entries.iter().enumerate() {
                let connector = if i == entries.len() - 1 {
                    "└─"
                } else {
                    "├─"
                };

                match &***child {
                    Node::Leaf { value } => {
                        let leaf_key = &key[..key.len() - 1]; // Remove the ":"
                        println!(
                            "{}{} LEAF -> key=\"{}\", value=\"{}\"",
                            prefix, connector, leaf_key, value
                        );
                    }
                    Node::Node { .. } => {
                        println!("{}{} NODE -> key=\"{}\"", prefix, connector, key);
                        let new_prefix = format!("{}  ", prefix);
                        Self::display_node_recursive(child, &new_prefix);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree = PrefixTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_single_insertion() {
        let mut tree = PrefixTree::new();
        tree.insert("bar", "val_bar".to_string());
        assert!(!tree.is_empty());
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.get("bar"), Some("val_bar".to_string()));
    }

    #[test]
    fn test_exact_user_scenario() {
        let mut tree = PrefixTree::new();

        println!("\n=== Testing User Scenario ===");

        // Step 1: Start with empty tree
        println!("1. Start with empty tree");
        tree.display_tree();

        // Step 2: Add "bar"
        println!("\n2. Add 'bar' value");
        tree.insert("bar", "val_bar".to_string());
        tree.display_tree();

        // Step 3: Add "foobar"
        println!("\n3. Add 'foobar' value");
        tree.insert("foobar", "val_foobar".to_string());
        tree.display_tree();

        // Step 4: Add "foofoo"
        println!("\n4. Add 'foofoo' value");
        tree.insert("foofoo", "val_foofoo".to_string());
        tree.display_tree();

        // Step 5: Add "foo"
        println!("\n5. Add 'foo' value");
        tree.insert("foo", "val_foo".to_string());
        tree.display_tree();

        // Verify all values
        assert_eq!(tree.get("bar"), Some("val_bar".to_string()));
        assert_eq!(tree.get("foobar"), Some("val_foobar".to_string()));
        assert_eq!(tree.get("foofoo"), Some("val_foofoo".to_string()));
        assert_eq!(tree.get("foo"), Some("val_foo".to_string()));
        assert_eq!(tree.len(), 4);

        println!("\n=== All tests passed! ===");
    }

    #[test]
    fn test_keys_collection() {
        let mut tree = PrefixTree::new();

        tree.insert("bar", "val_bar".to_string());
        tree.insert("foobar", "val_foobar".to_string());
        tree.insert("foofoo", "val_foofoo".to_string());
        tree.insert("foo", "val_foo".to_string());

        let mut keys = tree.keys();
        keys.sort();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys, vec!["bar", "foo", "foobar", "foofoo"]);
    }
}
