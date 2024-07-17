// Solution has
// Time complexity: O(1)
// Space complexity: O(1)
//
// A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
//
//     For example, the below binary watch reads "4:51".
//
// Given an integer turnedOn which represents the number of LEDs that are currently on (ignoring the PM), return all possible times the watch could represent. You may return the answer in any order.
//
// The hour must not contain a leading zero.
//
//     For example, "01:00" is not valid. It should be "1:00".
//
// The minute must consist of two digits and may contain a leading zero.
//
//     For example, "10:2" is not valid. It should be "10:02".

struct Solution;
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        for h in 0..12u32 {
            for min in 0..60u32 {
                if h.count_ones() + min.count_ones() == turned_on as u32 {
                    output.push(format!("{h}:{min:0>2}"));
                }
            }
        }
        output
    }
}
