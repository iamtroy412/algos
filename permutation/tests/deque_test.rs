use permutation::deque::*;

#[test]
fn new_deque() {
    let empty_deque = Deque::<i32>::new();
    assert!(empty_deque.is_empty());
    assert_eq!(0, empty_deque.size());
}
