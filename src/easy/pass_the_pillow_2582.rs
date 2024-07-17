// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// There are n people standing in a line labeled from 1 to n. The first person in the line is holding a pillow initially. Every second, the person holding the pillow passes it to the next person standing in the line. Once the pillow reaches the end of the line, the direction changes, and people continue passing the pillow in the opposite direction.
//
//     For example, once the pillow reaches the nth person they pass it to the n - 1th person, then to the n - 2th person and so on.
//
// Given the two positive integers n and time, return the index of the person holding the pillow after time seconds.
struct Solution;
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut go_forward = true;
        let mut pillow_position = 1;
        for _ in 0..time {
            if pillow_position == n {
                go_forward = false;
            } else if pillow_position == 1 {
                go_forward = true;
            }
            match go_forward {
                true => pillow_position += 1,
                false => pillow_position -= 1,
            }
        }
        pillow_position
    }
}
