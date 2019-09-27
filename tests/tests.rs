extern crate tree_rb;

#[cfg(test)]
use tree_rb::Tree;

#[test]
fn tree_new() {
    let tree: Tree<u32> = Tree::new();
    assert!(tree.is_empty());
}

#[test]
fn tree_insert_root() {
    let mut tree = Tree::new();
    tree.insert(5);
}
