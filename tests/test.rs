use rustlinqdemo1::sequence_extensions::{All, Any, FirstOrDefault, LastOrDefault, SkipTakeOwned, TakeOwned, TakeSlice};

#[test]
fn all_returns_false_when_one_item_does_not_match() {
    let values = vec![2, 4, 6, 7];

    assert!(!values.all(|value| *value % 2 == 0));
}

#[test]
fn any_returns_true_when_one_item_matches() {
    let values = vec![1, 3, 5, 8];

    assert!(values.any(|value| *value > 7));
}

#[test]
fn first_or_default_returns_first_item() {
    let values = vec![10, 20, 30];

    assert_eq!(values.first_or_default(), 10);
}

#[test]
fn last_or_default_returns_last_item() {
    let values = vec![10, 20, 30];

    assert_eq!(values.last_or_default(), 30);
}

#[test]
fn skip_take_owned_returns_requested_range() {
    let values = vec![1, 2, 3, 4, 5];

    assert_eq!(values.skip_take_owned(1, 2), vec![2, 3]);
}

#[test]
fn take_n_returns_borrowed_prefix() {
    let values = vec![1, 2, 3, 4, 5];

    assert_eq!(values.take_n(3), &[1, 2, 3]);
}

#[test]
fn take_owned_returns_owned_prefix() {
    let values = vec![1, 2, 3, 4, 5];

    assert_eq!(values.take_owned(4), vec![1, 2, 3, 4]);
}
