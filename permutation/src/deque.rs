use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
            prev: None,
        }
    }
}

pub struct Deque<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
}

impl<T> Deque<T> {
    // construct an empty deque
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    // is the deque empty?
    pub fn is_empty(&self) -> bool {
        // self.head == None
        self.length == 0
    }

    // return the number of items on the deque
    pub fn size(&self) -> usize {
        self.length
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
