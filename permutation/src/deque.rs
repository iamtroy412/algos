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

    // add the item to the back
    pub fn add_last(&mut self, item: T) {
        let new_tail = Node::new(item);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.length += 1;
    }

    // remove and return the item from the front
    pub fn remove_first(&mut self) -> Option<T> {
        self.length -= 1;
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}
