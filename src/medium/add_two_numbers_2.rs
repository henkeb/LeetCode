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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1_next: Option<&Box<ListNode>> = l1.as_ref();
        let mut l2_next: Option<&Box<ListNode>> = l2.as_ref();
        let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut current: Option<&mut Box<ListNode>> = result.as_mut();

        let mut carry = 0;
        while l1_next.is_some() || l2_next.is_some() {
            let mut sum = carry;

            if let Some(l1) = l1_next {
                sum += l1.val;
                l1_next = l1.next.as_ref();
            }

            if let Some(l2) = l2_next {
                sum += l2.val;
                l2_next = l2.next.as_ref();
            }

            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;
            current = current.unwrap().next.as_mut();
        }
        if carry == 1 {
            current.unwrap().as_mut().next = Some(Box::new(ListNode::new(carry)));
        }
        result.unwrap().next
    }
}
