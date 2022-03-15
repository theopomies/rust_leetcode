// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (&l1, &l2) {
            (Some(_), Some(_)) => Self::add(l1, l2, 0),
            (Some(_), _) => l1,
            (_, Some(_)) => l2,
            _ => None,
        }
    }

    fn add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        match (&l1, &l2, carry) {
            (None, None, 0) => None,
            (None, None, carry) => Some(Box::new(ListNode::new(carry))),
            _ => {
                let l1 = l1.unwrap_or(Box::new(ListNode::new(0)));
                let l2 = l2.unwrap_or(Box::new(ListNode::new(0)));
                let sum = carry + l1.val + l2.val;
                Some(Box::new(ListNode {
                    val: sum % 10,
                    next: Self::add(l1.next, l2.next, sum / 10),
                }))
            }
        }
    }
}
