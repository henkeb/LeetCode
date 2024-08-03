// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// A certain bug's home is on the x-axis at position x. Help them get there from position 0.
//
// The bug jumps according to the following rules:
//
//     It can jump exactly a positions forward (to the right).
//     It can jump exactly b positions backward (to the left).
//     It cannot jump backward twice in a row.
//     It cannot jump to any forbidden positions.
//
// The bug may jump forward beyond its home, but it cannot jump to positions numbered with negative integers.
//
// Given an array of integers forbidden, where forbidden[i] means that the bug cannot jump to the position forbidden[i], and integers a, b, and x, return the minimum number of jumps needed for the bug to reach its home. If there is no possible sequence of jumps that lands the bug on position x, return -1.

struct Solution;
use std::collections::VecDeque;
#[derive(Clone, PartialEq)]
enum Visited {
    Not,
    Once,
    Forbidden,
}
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let a = a as usize;
        let b = b as usize;
        let x = x as usize;
        let max_jump = 6000;
        let mut visited: Vec<Visited> = vec![Visited::Not; max_jump];
        for &x in forbidden.iter() {
            visited[x as usize] = Visited::Forbidden;
        }
        let mut queue: VecDeque<(usize, bool, usize)> = VecDeque::new();
        queue.push_back((0, true, 0));
        while let Some((position, prev_forward_jump, jumps)) = queue.pop_front() {
            if position + a == x {
                return jumps as i32 + 1;
            }

            if position + a < max_jump && visited[position + a] != Visited::Forbidden {
                visited[position + a] = Visited::Forbidden;
                queue.push_back((position + a, true, jumps + 1));
            }

            if prev_forward_jump && position > b {
                if position - b == x {
                    return jumps as i32 + 1;
                }
                if visited[position - b] == Visited::Not {
                    visited[position - b] = Visited::Once;
                    queue.push_back((position - b, false, jumps + 1));
                }
            }
        }
        -1
    }
}
