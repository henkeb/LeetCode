// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//
// You are given a string s and two integers x and y. You can perform two types of operations any number of times.
//
//     Remove substring "ab" and gain x points.
//         For example, when removing "ab" from "cabxbae" it becomes "cxbae".
//     Remove substring "ba" and gain y points.
//         For example, when removing "ba" from "cabxbae" it becomes "cabxe".
//
// Return the maximum points you can gain after applying the above operations on s.
struct Solution;
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        use std::cmp::Ordering;
        let mut stack: Vec<char> = Vec::new();

        let (most_points, less_points): (char, char) = match x.cmp(&y) {
            Ordering::Greater | Ordering::Equal => ('a', 'b'),
            Ordering::Less => ('b', 'a'),
        };
        let mut points = 0;

        for c in s.chars() {
            match c {
                c if c == most_points => stack.push(c),
                c if c == less_points => {
                    if stack.last() != None && *stack.last().unwrap() == most_points {
                        stack.pop();
                        points += x.max(y);
                    } else {
                        stack.push(c);
                    }
                }
                _ => {
                    points += x.min(y) * Self::clean_stack(&mut stack, most_points);
                }
            }
        }
        points += x.min(y) * Self::clean_stack(&mut stack, most_points);
        points
    }

    fn clean_stack(stack: &mut Vec<char>, most_points: char) -> i32 {
        let mut num_most_points: i32 = 0;
        let mut less_points_pair: i32 = 0;

        while let Some(c) = stack.pop() {
            if c == most_points {
                num_most_points += 1
            } else if num_most_points > 0 {
                less_points_pair += 1;
                num_most_points -= 1;
            }
        }

        less_points_pair
    }
}
