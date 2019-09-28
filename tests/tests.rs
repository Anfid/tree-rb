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

#[test]
fn tree_iter() {
    let mut tree = tree_rb::Tree::new();
    tree.insert(15)
        .insert(10)
        .insert(15)
        .insert(17)
        .insert(44)
        .insert(0);

    let z: Vec<i32> = tree
        .into_iter()
        .map(|x| x * 2)
        .filter(|x| *x > 20)
        .take(3)
        .collect();
}
