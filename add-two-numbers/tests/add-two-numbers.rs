use add_two_numbers::{ListNode, Solution};

fn list_from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut res = None;
    for val in vec {
        res = Some(Box::new(ListNode { next: res, val }))
    }
    res
}

#[test]
fn example1() {
    let l1 = list_from_vec(vec![2, 4, 3]);
    let l2 = list_from_vec(vec![5, 6, 4]);
    let expected = list_from_vec(vec![8, 0, 7]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(expected, result);
}

#[test]
fn example2() {
    let l1 = list_from_vec(vec![0]);
    let l2 = list_from_vec(vec![0]);
    let expected = list_from_vec(vec![0]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(expected, result);
}

#[test]
fn example3() {
    let l1 = list_from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = list_from_vec(vec![9, 9, 9, 9]);
    let expected = list_from_vec(vec![1, 0, 0, 0, 9, 9, 9, 8]);
    let result = Solution::add_two_numbers(l1, l2);
    assert_eq!(expected, result);
}
