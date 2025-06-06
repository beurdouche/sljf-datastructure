use sljf_datastructure::MerklePatriciaTree;

fn main() {
    println!("Merkle Patricia Tree Demo");
    println!("========================");

    // Create a new tree
    let mut tree = MerklePatriciaTree::new();

    // Insert the keys from the diagram
    println!("\n1. Inserting key-value pairs...");
    tree.insert(b"foo", b"Value at 'foo'".to_vec()).unwrap();
    tree.insert(b"foobar", b"Value at 'foobar'".to_vec())
        .unwrap();
    tree.insert(b"foofoo", b"Value at 'foofoo'".to_vec())
        .unwrap();
    tree.insert(b"bar", b"Value at 'bar'".to_vec()).unwrap();

    println!("   ✓ Inserted 4 key-value pairs");
    println!("   ✓ Tree size: {}", tree.len());

    // Display the root hash
    if let Some(root_hash) = tree.root_hash() {
        println!("   ✓ Root hash: {}", hex::encode(root_hash));
    }

    // Retrieve values
    println!("\n2. Retrieving values...");
    let keys = [&b"foo"[..], &b"foobar"[..], &b"foofoo"[..], &b"bar"[..]];
    for key in &keys {
        if let Some(value) = tree.get(key) {
            println!(
                "   {} -> {}",
                String::from_utf8_lossy(key),
                String::from_utf8_lossy(&value)
            );
        }
    }

    // Test non-existent key
    println!("\n3. Testing non-existent key...");
    if tree.get(b"baz").is_none() {
        println!("   ✓ 'baz' not found (as expected)");
    }

    // Verify tree integrity
    println!("\n4. Verifying tree integrity...");
    if tree.verify_integrity() {
        println!("   ✓ Tree integrity verified");
    } else {
        println!("   ✗ Tree integrity check failed");
    }

    // List all keys
    println!("\n5. All keys in the tree:");
    let all_keys = tree.keys();
    for key in &all_keys {
        println!("   - {}", String::from_utf8_lossy(key));
    }

    // Display the tree structure
    println!("\n6. Tree structure visualization:");
    tree.display_tree();

    // Demonstrate prefix sharing efficiency
    println!("\n7. Demonstrating prefix sharing...");
    println!("   Keys 'foo', 'foobar', and 'foofoo' share the prefix 'foo'");
    println!("   The tree efficiently stores this shared prefix only once");
    println!("   in extension nodes, reducing memory usage.");

    // Update an existing key
    println!("\n8. Updating existing key...");
    let old_value = tree.get(b"foo").unwrap();
    tree.insert(b"foo", b"Updated value for 'foo'".to_vec())
        .unwrap();
    let new_value = tree.get(b"foo").unwrap();

    println!("   Old value: {}", String::from_utf8_lossy(&old_value));
    println!("   New value: {}", String::from_utf8_lossy(&new_value));
    println!("   Tree size remains: {} (no new key added)", tree.len());

    println!("\nDemo completed successfully! 🎉");
}
