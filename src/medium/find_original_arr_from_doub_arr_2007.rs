use std::collections::HashMap;

// An integer array original is transformed into a doubled array changed by appending twice the value of every element in original, and then randomly shuffling the resulting array.
//
// Given an array changed, return original if changed is a doubled array. If changed is not a doubled array, return an empty array. The elements in original may be returned in any order.
//
//
//
// Example 1:
//
// Input: changed = [1,3,4,2,6,8]
// Output: [1,3,4]
// Explanation: One possible original array could be [1,3,4]:
// - Twice the value of 1 is 1 * 2 = 2.
// - Twice the value of 3 is 3 * 2 = 6.
// - Twice the value of 4 is 4 * 2 = 8.
// Other original arrays could be [4,3,1] or [3,1,4].
// Example 2:
//
// Input: changed = [6,3,0,1]
// Output: []
// Explanation: changed is not a doubled array.
// Example 3:
//
// Input: changed = [1]
// Output: []
// Explanation: changed is not a doubled array.
//
//
// Constraints:
//
// 1 <= changed.length <= 105
// 0 <= changed[i] <= 105
//
// Solution has
// Time complexity: O(n*Log(n))
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        if changed.len() <= 1 {
            return vec![];
        }
        let mut odd: HashMap<i32, i32> = HashMap::new();
        let mut even: HashMap<i32, i32> = HashMap::new();
        let mut output: Vec<i32> = Vec::new();

        for &val in changed.iter() {
            if val & 1 == 0 {
                let _ = *even.entry(val).and_modify(|v| *v += 1).or_insert(1);
            } else {
                let _ = *odd.entry(val).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        for (num, val) in odd {
            if let Some(even_val) = even.get_mut(&(num * 2)) {
                if *even_val < val {
                    return vec![];
                }
                for _ in 0..val {
                    output.push(num);
                }
                if *even_val == val {
                    even.remove(&(num * 2));
                } else {
                    *even_val -= val;
                }
            } else {
                return vec![];
            }
        }

        let mut even_arr: Vec<i32> = Vec::new();

        for (key, val) in even.iter() {
            for _ in 0..*val {
                even_arr.push(*key);
            }
        }

        even_arr.sort_unstable();

        while !even_arr.is_empty() {
            if even_arr[0] == 0 {
                if even_arr.iter().filter(|&val| *val == 0).count() & 1 == 1 {
                    return vec![];
                }
            }
            if let Some(double) = even_arr.iter().position(|&val| val == even_arr[0] * 2) {
                output.push(even_arr[0]);
                even_arr.remove(double);
                even_arr.remove(0);
            } else {
                return vec![];
            }
        }
        output
    }
}
