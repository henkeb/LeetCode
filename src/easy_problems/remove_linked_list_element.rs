// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
//Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.
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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut current = &mut dummy;
        while let Some(node) = current.next.as_mut() {
            if node.val == val {
                let next_node = node.next.take();
                current.next = next_node;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_remove_elements() {
        assert_eq!(
            Solution::remove_elements(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 6,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode {
                                    val: 4,
                                    next: Some(Box::new(ListNode {
                                        val: 5,
                                        next: Some(Box::new(ListNode { val: 6, next: None }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                })),
                6
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 5, next: None }))
                        }))
                    }))
                }))
            }))
        );
    }
}
