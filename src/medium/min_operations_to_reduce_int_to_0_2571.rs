// Solution has
// Time complexity: O(logn)
// Space complexity: O(1)
//
// You are given a positive integer n, you can do the following operation any number of times:
//
//     Add or subtract a power of 2 from n.
//
// Return the minimum number of operations to make n equal to 0.
//
// A number x is power of 2 if x == 2i where i >= 0.
struct Solution;
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n > 0 {
            while n & 1 == 0 {
                n >>= 1;
            }
            count += 1;
            if n & 2 == 0 {
                n -= 1;
            } else {
                n += 1;
            }
        }
        count
    }
}
