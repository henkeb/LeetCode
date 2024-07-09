// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// /A critical point in a linked list is defined as either a local maxima or a local minima.
//
// A node is a local maxima if the current node has a value strictly greater than the previous node and the next node.
//
// A node is a local minima if the current node has a value strictly smaller than the previous node and the next node.
//
// Note that a node can only be a local maxima/minima if there exists both a previous node and a next node.
//
// Given a linked list head, return an array of length 2 containing [minDistance, maxDistance] where minDistance is the minimum distance between any two distinct critical points and maxDistance is the maximum distance between any two distinct critical points. If there are fewer than two critical points, return [-1, -1]./
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
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut prev: &Option<Box<ListNode>> = &None;
        let mut curr = &head;
        let mut max_min_points: Vec<usize> = Vec::new();
        let mut node_count = 0;
        while let Some(curr_node) = curr {
            node_count += 1;
            let next = &curr_node.as_ref().next;
            if next.is_some() && prev.is_some() {
                let next_val = &next.as_ref().unwrap().val;
                let prev_val = &prev.as_ref().unwrap().val;
                if *prev_val > curr_node.val && *next_val > curr_node.val {
                    max_min_points.push(node_count);
                }
                if *prev_val < curr_node.val && *next_val < curr_node.val {
                    max_min_points.push(node_count);
                }
            }
            prev = curr;
            curr = next;
        }
        let min = max_min_points
            .windows(2)
            .map(|window| (window[1] - window[0]) as i32)
            .min();
        let max = match (max_min_points.last(), max_min_points.first()) {
            (Some(first), Some(last)) if first != last => (first - last) as i32,
            (_, _) => -1,
        };
        vec![min.unwrap_or(-1), max]
    }
}
