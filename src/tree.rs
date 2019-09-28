use crate::node::Node;
use crate::node::Side::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(Default)]
pub struct Tree<T: Ord> {
    root: Option<*mut Node<T>>,
    size: usize,
}

impl<T: Ord> Tree<T> {
    /// Constructs a new, empty `Tree<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tree_rb::Tree;
    /// let mut tree: Tree<i32> = Tree::new();
    /// ```
    pub fn new() -> Tree<T> {
        Tree {
            root: None,
            size: 0,
        }
    }

    /// Checks if tree contains any data.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tree_rb::Tree;
    /// let mut tree: Tree<i32> = Tree::new();
    /// assert_eq!(tree.is_empty(), true);
    ///
    /// tree.insert(0);
    /// assert_eq!(tree.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Insert data in the tree.
    /// (TODO: Tree should be rebalanced after this operation)
    /// (TODO: Increment size)
    /// (TODO: Test performance)
    /// ```
    /// # use tree_rb::Tree;
    /// let mut tree: Tree<i32> = Tree::new();
    ///
    /// tree.insert(8);
    /// tree.insert(2);
    /// ```
    pub fn insert(&mut self, data: T) -> &mut Self {
        if let Some(mut parent) = self.root {
            loop {
                match unsafe { (*parent).data.cmp(&data) } {
                    Ordering::Less => unsafe {
                        if let Some(n) = (*parent).child(Right) {
                            parent = n;
                        } else {
                            self.size += 1;
                            Node::with_parent(data, parent, Right);
                            break;
                        }
                    },
                    Ordering::Greater => unsafe {
                        if let Some(n) = (*parent).child(Left) {
                            parent = n;
                        } else {
                            self.size += 1;
                            Node::with_parent(data, parent, Left);
                            break;
                        }
                    },
                    Ordering::Equal => break,
                }
            }
        } else {
            self.root = Some(unsafe { Node::new(data) });
        }

        //self.insert_rebalance(&mut *new_node);

        self
    }

    /// Insert data in the tree, implemented recursively
    /// (TODO: Tree should be rebalanced after this operation)
    /// (TODO: Increment size)
    /// (TODO: Test performance)
    /// ```
    /// # use tree_rb::Tree;
    /// let mut tree: Tree<i32> = Tree::new();
    ///
    /// tree.insert_rec(8);
    /// tree.insert_rec(2);
    /// ```
    pub fn insert_rec(&mut self, data: T) -> &mut Self {
        if let Some(root) = self.root {
            unsafe {
                (*root).insert(data);
            }
        } else {
            self.root = Some(unsafe { Node::new(data) })
        }

        self
    }
}

impl<T: Ord + Copy> Tree<T> {
    fn in_order_traverse(self) -> Vec<T> {
        let mut vec = Vec::new();
        vec.reserve(self.size);

        if let Some(root) = self.root {
            unsafe {
                (*root).in_order_traverse(&mut vec);
            }
        }

        vec
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(root) = self.root {
            unsafe { write!(f, "{}", *root) }
        } else {
            write!(f, "[]")
        }
    }
}

impl<T: Ord> Drop for Tree<T> {
    fn drop(&mut self) {
        if let Some(r) = self.root {
            unsafe {
                Box::from_raw(r);
            }
        }
    }
}

impl<T: Ord + Copy> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> std::vec::IntoIter<T> {
        self.in_order_traverse().into_iter()
    }
}
