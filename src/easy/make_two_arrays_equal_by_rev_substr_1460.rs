// Solution has
// Time complexity: O(?)
// Space complexity: O(?)
//
// You are given two integer arrays of equal length target and arr. In one step, you can select any non-empty subarray of arr and reverse it. You are allowed to make any number of steps.
//
// Return true if you can make arr equal to target or false otherwise.
struct Solution;
impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        use std::collections::HashMap;
        let mut arr_hm: HashMap<i32, i32> = HashMap::new();
        for &num in arr.iter() {
            arr_hm.entry(num).and_modify(|val| *val += 1).or_insert(1);
        }
        for &num in target.iter() {
            match arr_hm.get_mut(&num) {
                None => return false,
                Some(val) => {
                    *val -= 1;
                    if *val == 0 {
                        arr_hm.remove(&num);
                    }
                }
            }
        }
        true
    }
}
