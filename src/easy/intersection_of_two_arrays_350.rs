// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.
struct Solution;
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::cmp::min;
        use std::cmp::Ordering;
        use std::collections::HashMap;

        let (mut longer, mut shorter): (&Vec<i32>, &Vec<i32>) = (&nums1, &nums2);
        match nums1.len().cmp(&nums2.len()) {
            Ordering::Greater => (),
            Ordering::Equal => (),
            Ordering::Less => {
                longer = &nums2;
                shorter = &nums1;
            }
        }
        let mut l_hashmap: HashMap<i32, i32> = HashMap::new();
        let mut r_hashmap: HashMap<i32, i32> = HashMap::new();
        longer.iter().for_each(|&val| match l_hashmap.get(&val) {
            Some(v) => {
                l_hashmap.insert(val, v + 1);
            }
            None => {
                l_hashmap.insert(val, 1);
            }
        });
        shorter.iter().for_each(|&val| match r_hashmap.get(&val) {
            Some(v) => {
                r_hashmap.insert(val, v + 1);
            }
            None => {
                r_hashmap.insert(val, 1);
            }
        });

        let mut output: Vec<i32> = Vec::new();
        r_hashmap
            .iter()
            .for_each(|(key, r_value)| match l_hashmap.get(key) {
                None => (),
                Some(l_value) => {
                    (0..min(*r_value, *l_value)).for_each(|_| output.push(*key));
                }
            });
        output
    }
}
