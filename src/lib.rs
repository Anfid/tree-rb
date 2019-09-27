use std::alloc::{alloc, Layout};
use std::fmt;

#[derive(Default)]
pub struct Tree<T: Ord> {
    root: Option<*mut Node<T>>,
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
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Insert data in the tree. (TODO: Tree should be rebalanced after this operation)
    pub fn insert(&mut self, data: T) -> &mut Self {
        unsafe {
            let new_node: *mut Node<T> = alloc(Layout::new::<Node<T>>()) as *mut Node<T>;

            let mut parent = None;
            let mut node = self.root;
            while let Some(n) = node {
                if (*n).data > data {
                    parent = node;
                    node = (*n).left;
                } else {
                    parent = node;
                    node = (*n).right;
                }
            }

            if let Some(p) = parent {
                if (*p).data > data {
                    (*p).left = Some(new_node);
                } else {
                    (*p).right = Some(new_node);
                }
                *new_node = Node::with_parent(data, p);
            } else {
                self.root = Some(new_node);
                *new_node = Node::new(data);
            }
        }

        self
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            if let Some(root) = self.root {
                write!(f, "{}", *root)
            } else {
                write!(f, "[]")
            }
        }
    }
}

// Currently unimplemented, so memleaks are expected
impl<T: Ord> Drop for Tree<T> {
    fn drop(&mut self) {
        // TODO
    }
}

#[allow(unused)]
enum Color {
    Red,
    Black,
}

#[allow(unused)]
struct Node<T: Ord> {
    pub data: T,
    color: Color,
    parent: Option<*const Node<T>>,
    left: Option<*mut Node<T>>,
    right: Option<*mut Node<T>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(data: T) -> Node<T> {
        Node {
            data,
            color: Color::Black,
            parent: None,
            left: None,
            right: None,
        }
    }

    fn with_parent(data: T, parent: *mut Node<T>) -> Node<T> {
        Node {
            data,
            color: Color::Red,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, _data: T) {
        // TODO
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(f, "[")?;
            if let Some(l) = self.left {
                write!(f, "{}", *l)?;
            }
            write!(f, "{}", self.data)?;
            if let Some(r) = self.right {
                write!(f, "{}", *r)?;
            }
            write!(f, "]")
        }
    }
}
