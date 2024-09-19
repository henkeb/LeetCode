// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given the head of a linked list head, in which each node contains an integer value.
//
// Between every pair of adjacent nodes, insert a new node with a value equal to the greatest common divisor of them.
//
// Return the linked list after insertion.
//
// The greatest common divisor of two numbers is the largest positive integer that evenly divides both numbers.
//
//
//
// Example 1:
//
// Input: head = [18,6,10,3]
// Output: [18,6,6,2,10,1,3]
// Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes (nodes in blue are the inserted nodes).
// - We insert the greatest common divisor of 18 and 6 = 6 between the 1st and the 2nd nodes.
// - We insert the greatest common divisor of 6 and 10 = 2 between the 2nd and the 3rd nodes.
// - We insert the greatest common divisor of 10 and 3 = 1 between the 3rd and the 4th nodes.
// There are no more adjacent nodes, so we return the linked list.
//
// Example 2:
//
// Input: head = [7]
// Output: [7]
// Explanation: The 1st diagram denotes the initial linked list and the 2nd diagram denotes the linked list after inserting the new nodes.
// There are no pairs of adjacent nodes, so we return the initial linked list.
//
//
//
// Constraints:
//
//     The number of nodes in the list is in the range [1, 5000].
//     1 <= Node.val <= 1000
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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut current = &mut head;

        while let Some(node) = current {
            if node.next.is_none() {
                break;
            }
            match &node.next {
                Some(next) => {
                    let gcd = Self::gcd(node.val, next.val);
                    let next = node.next.take();
                    node.next = Some(Box::new(ListNode { next, val: gcd }));
                }
                None => (),
            };
            current = &mut node.next.as_mut().unwrap().next;
        }
        head
    }

    fn gcd(a: i32, b: i32) -> i32 {
        let mut a: i32 = a;
        let mut b: i32 = b;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}
