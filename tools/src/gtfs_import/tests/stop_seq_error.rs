use crate::utils::stop_seq_error;

#[test]
fn empty() {
    let vec1: Vec<i32> = vec![];
    let vec2 = vec![];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 0);
    assert_eq!(mismatches, 0);
}

#[test]
fn same_element() {
    let vec1 = vec![0];
    let vec2 = vec![0];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 1);
    assert_eq!(mismatches, 0);
}

#[test]
fn different_elements() {
    let vec1 = vec![0];
    let vec2 = vec![1];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 0);
    assert_eq!(mismatches, 1);
}

#[test]
fn different_elements_reverse() {
    let vec1 = vec![1];
    let vec2 = vec![0];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 0);
    assert_eq!(mismatches, 1);
}

#[test]
fn first_half_of_sequence() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3, 4, 5, 6];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 3);
    assert_eq!(mismatches, 3);
}

#[test]
fn second_half_of_sequence() {
    let vec1 = vec![4, 5, 6];
    let vec2 = vec![1, 2, 3, 4, 5, 6];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 3);
    assert_eq!(mismatches, 3);
}

#[test]
fn middle_of_sequence() {
    let vec1 = vec![3, 4];
    let vec2 = vec![1, 2, 3, 4, 5, 6];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 2);
    assert_eq!(mismatches, 4);
}

#[test]
fn completely_different() {
    let vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![5, 6, 7, 8];
    let (matches, mismatches) = stop_seq_error(&vec1, &vec2);
    assert_eq!(matches, 0);
    assert_eq!(mismatches, 4);
}
