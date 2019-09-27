use std::fmt;

#[allow(unused)]
pub(crate) enum Color {
    Red,
    Black,
}

#[allow(unused)]
pub(crate) struct Node<T: Ord> {
    pub(crate) data: T,
    pub(crate) color: Color,
    pub(crate) parent: Option<*const Node<T>>,
    pub(crate) left: Option<*mut Node<T>>,
    pub(crate) right: Option<*mut Node<T>>,
}

impl<T> Node<T>
where
    T: Ord,
{
    pub(crate) fn new(data: T) -> Node<T> {
        Node {
            data,
            color: Color::Red,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub(crate) fn with_parent(data: T, parent: *mut Node<T>) -> Node<T> {
        Node {
            data,
            color: Color::Red,
            parent: Some(parent),
            left: None,
            right: None,
        }
    }

    pub(crate) fn insert(&mut self, _data: T) {
        // TODO
    }
}

impl<T: Ord + fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        if let Some(l) = self.left {
            unsafe {
                write!(f, "{}", *l)?;
            }
        }
        write!(f, "{}", self.data)?;
        if let Some(r) = self.right {
            unsafe {
                write!(f, "{}", *r)?;
            }
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
