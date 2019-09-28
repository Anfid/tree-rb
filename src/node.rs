use std::alloc::{alloc, Layout};
use std::cmp::Ordering;
use std::fmt;
use std::ops::Not;

pub(crate) enum Color {
    Red,
    Black,
}

pub(crate) use Side::*;
pub(crate) enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Side {
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

pub(crate) struct Parent<T: Ord> {
    my_side: Side,
    addr: *mut Node<T>,
}

pub(crate) struct Node<T: Ord> {
    pub(crate) data: T,
    pub(crate) color: Color,
    pub(crate) parent: Option<Parent<T>>,
    pub(crate) left: Option<*mut Node<T>>,
    pub(crate) right: Option<*mut Node<T>>,
}

impl<T: Ord> Node<T> {
    /// Create Black root node without parent. Since parent manages
    /// node lifetime, this function is marked `unsafe`. This node has to
    /// be set as tree root to prevent memory leaks.
    pub(crate) unsafe fn new(data: T) -> *mut Node<T> {
        let new: *mut Node<T> = alloc(Layout::new::<Node<T>>()) as *mut Node<T>;

        *new = Node {
            data,
            color: Color::Black,
            parent: None,
            left: None,
            right: None,
        };

        new
    }

    /// Create Red child node with given parent, if possible. Silently
    /// fails otherwise. Sets both pointers, so node lifetime will be
    /// managed by parent.
    pub(crate) fn with_parent(data: T, parent: *mut Node<T>, side: Side) -> *mut Node<T> {
        unsafe {
            let new: *mut Node<T> = alloc(Layout::new::<Node<T>>()) as *mut Node<T>;

            (*(*parent).child_raw(&side)) = Some(new);

            *new = Node {
                data,
                color: Color::Red,
                parent: Some(Parent {
                    my_side: side,
                    addr: parent,
                }),
                left: None,
                right: None,
            };

            new
        }
    }

    /// Recursive insert function.
    pub(crate) fn insert(&mut self, data: T) {
        match self.data.cmp(&data) {
            Ordering::Less => unsafe {
                if let Some(node) = self.right {
                    (*node).insert(data)
                } else {
                    Node::with_parent(data, self, Right);
                }
            },
            Ordering::Greater => unsafe {
                if let Some(node) = self.left {
                    (*node).insert(data)
                } else {
                    Node::with_parent(data, self, Left);
                }
            },
            Ordering::Equal => {}
        }
    }

    /// Returns child reference.
    pub(crate) fn child(&self, s: Side) -> Option<&mut Node<T>> {
        match s {
            Left => self.left.map(|ptr| unsafe { &mut *ptr }),
            Right => self.right.map(|ptr| unsafe { &mut *ptr }),
        }
    }

    /// Returns raw child pointer.
    pub(crate) fn child_raw(&mut self, s: &Side) -> &mut Option<*mut Node<T>> {
        match s {
            Left => &mut self.left,
            Right => &mut self.right,
        }
    }
}

impl<T: Ord + Copy> Node<T> {
    pub(crate) fn in_order_traverse(&self, acc: &mut Vec<T>) {
        if let Some(l) = self.child(Left) {
            l.in_order_traverse(acc);
        }
        acc.push(self.data);
        if let Some(r) = self.child(Right) {
            r.in_order_traverse(acc);
        }
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        if let Some(l) = self.child(Left) {
            write!(f, "{}", l)?;
        }
        write!(f, "{}", self.data)?;
        if let Some(r) = self.child(Right) {
            write!(f, "{}", r)?;
        }
        write!(f, "]")
    }
}

impl<T: Ord> Drop for Node<T> {
    fn drop(&mut self) {
        if let Some(l) = self.left {
            unsafe {
                Box::from_raw(l);
            }
        }
        if let Some(r) = self.right {
            unsafe {
                Box::from_raw(r);
            }
        }
    }
}
