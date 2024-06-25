// Given a string s, return the number of segments in the string.
//
// A segment is defined to be a contiguous sequence of non-space characters.
struct Solution;
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }
}
