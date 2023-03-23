use permutation::deque::*;

#[test]
fn empty_deque_test() {
    let empty_deque = Deque::<i32>::new();
    assert!(empty_deque.is_empty());
    assert_eq!(0, empty_deque.size());
}

#[test]
fn add_first_test() {
    let mut deque = Deque::<i32>::new();

    for i in 0..10 {
        deque.add_frist(i);
    }

    assert_eq!(false, deque.is_empty());
    assert_eq!(10, deque.size());
}

#[test]
fn add_last_test() {
    let mut deque = Deque::<i32>::new();

    for i in 0..20 {
        deque.add_last(i);
    }

    assert_eq!(false, deque.is_empty());
    assert_eq!(20, deque.size());
}

#[test]
fn remove_first_test() {
    let mut deque = Deque::<i32>::new();

    // removing from an empty deque should return `None`
    assert_eq!(None, deque.remove_first());

    for i in 0..10 {
        deque.add_frist(i);
    }

    for i in 9..=0 {
        assert_eq!(i, deque.remove_first().unwrap());
    }
}
