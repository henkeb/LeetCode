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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        let mut next_head: Option<Box<ListNode>> = None;

        if let Some(node) = head {
            new_head = Some(Box::new(ListNode::new(node.val)));
            next_head = node.next;
        }

        while let Some(node) = next_head {
            let temp = new_head;
            new_head = Some(Box::new(ListNode::new(node.val)));
            new_head.as_mut().unwrap().next = temp;
            next_head = node.next;
        }
        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_list(Some(Box::new(ListNode::new(1)))),
            Some(Box::new(ListNode::new(1)))
        );
    }

    #[test]
    fn it_works2() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        let mut res = Some(Box::new(ListNode::new(2)));
        res.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        assert_eq!(Solution::reverse_list(head), res);
    }
}
