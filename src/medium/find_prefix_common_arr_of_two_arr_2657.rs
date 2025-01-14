use std::collections::HashSet;

// You are given two 0-indexed integer permutations A and B of length n.
//
// A prefix common array of A and B is an array C such that C[i] is equal to the count of numbers that are present at or before the index i in both A and B.
//
// Return the prefix common array of A and B.
//
// A sequence of n integers is called a permutation if it contains all integers from 1 to n exactly once.
//
//
//
// Example 1:
//
// Input: A = [1,3,2,4], B = [3,1,2,4]
// Output: [0,2,3,4]
// Explanation: At i = 0: no number is common, so C[0] = 0.
// At i = 1: 1 and 3 are common in A and B, so C[1] = 2.
// At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.
// At i = 3: 1, 2, 3, and 4 are common in A and B, so C[3] = 4.
// Example 2:
//
// Input: A = [2,3,1], B = [3,1,2]
// Output: [0,1,3]
// Explanation: At i = 0: no number is common, so C[0] = 0.
// At i = 1: only 3 is common in A and B, so C[1] = 1.
// At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.
//
//
// Constraints:
//
// 1 <= A.length == B.length == n <= 50
// 1 <= A[i], B[i] <= n
// It is guaranteed that A and B are both a permutation of n integers.
//
// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
struct Solution;
impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_set: HashSet<i32> = HashSet::new();
        let mut b_set: HashSet<i32> = HashSet::new();
        let mut output = Vec::with_capacity(a.len());
        let mut count = 0;
        a.iter().zip(b).for_each(|(&a_val, b_val)| {
            if a_val == b_val {
                count += 1;
            } else {
                if b_set.contains(&a_val) {
                    count += 1;
                    b_set.remove(&a_val);
                } else {
                    a_set.insert(a_val);
                }
                if a_set.contains(&b_val) {
                    count += 1;
                    a_set.remove(&b_val);
                } else {
                    b_set.insert(b_val);
                }
            }
            output.push(count);
        });
        output
    }
}
