// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut numbers: Vec<i32> = Vec::new();
        let mut current = &head;
        while let Some(node) = current {
            numbers.push(node.val);
            current = &node.next;
        }

        for i in 0..numbers.len() / 2 {
            if numbers[i] != numbers[numbers.len() - 1 - i] {
                return false;
            }
        }

        true
    }
}
