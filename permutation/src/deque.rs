use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(t: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val: t,
            next: None,
            prev: None,
        }))
    }
}

pub struct Deque<T> {
    length: usize,
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Deque<T> {
    // construct an empty deque
    pub fn new() -> Self {
        Deque {
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

    // add the item to the front
    pub fn add_frist(&mut self, item: T) {
        let new_head = Node::new(item);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
        self.length += 1;
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
