// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// /You are given the head of a linked list, which contains a series of integers separated by 0's. The beginning and end of the linked list will have Node.val == 0.
//
// For every two consecutive 0's, merge all the nodes lying in between them into a single node whose value is the sum of all the merged nodes. The modified list should not contain any 0's.
//
// Return the head of the modified linked list./
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
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_deref_mut();
        while let Some(curr_node) = curr.take() {
            let Some(mut next_node) = curr_node.next.take() else {
                return head;
            };
            let tail = next_node.next.take();
            curr_node.next = tail;
            if next_node.val == 0 {
                curr = curr_node.next.as_deref_mut();
            } else {
                curr_node.val += next_node.val;
                curr = Some(curr_node);
            }
        }
        head
    }
}
