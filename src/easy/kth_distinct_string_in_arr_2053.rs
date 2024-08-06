// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// A distinct string is a string that is present only once in an array.
//
// Given an array of strings arr, and an integer k, return the kth distinct string present in arr. If there are fewer than k distinct strings, return an empty string "".
//
// Note that the strings are considered in the order in which they appear in the array.
struct Solution;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        use std::collections::HashSet;
        let mut seen: HashSet<&str> = HashSet::new();
        let mut distinct: HashSet<&str> = HashSet::new();

        for s in arr.iter() {
            if !seen.insert(s) {
                distinct.remove(&s.as_str());
            } else {
                distinct.insert(s);
            }
        }

        for s in arr.iter() {
            if distinct.contains(s.as_str()) {
                k -= 1;
                if k == 0 {
                    return s.to_string();
                }
            }
        }
        "".to_string()
    }
}
