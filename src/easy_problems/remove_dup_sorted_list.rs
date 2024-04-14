// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
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

type Node = Option<Box<ListNode>>;
struct Solution;
impl Solution {
    pub fn delete_duplicates(mut head: Node) -> Node {
        let mut current_node = &mut head;
        while let Some(node) = current_node {
            while let Some(next) = &mut node.next {
                if node.val == next.val {
                    node.next = next.next.take();
                } else {
                    break;
                }
            }
            current_node = &mut node.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_delete_duplicates() {
        assert_eq!(Solution::delete_duplicates(None), None);
    }
}
