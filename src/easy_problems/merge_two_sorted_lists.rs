// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
//You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
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
struct Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::cmp::Ordering;
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
            (Some(l1), Some(l2)) => match l1.val.cmp(&l2.val) {
                Ordering::Less | Ordering::Equal => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
                Ordering::Greater => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(l2.next, Some(l1)),
                })),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge_two_lists(Some(Box::new(ListNode::new(1))), None),
            Some(Box::new(ListNode::new(1)))
        );
    }

    #[test]
    fn test3() {
        let mut l1: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        let l2 = Some(Box::new(ListNode::new(2)));

        let mut merged: Option<Box<ListNode>> = Some(Box::new(ListNode::new(1)));
        merged.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        merged.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));

        assert_eq!(Solution::merge_two_lists(l1, l2), merged)
    }
}
