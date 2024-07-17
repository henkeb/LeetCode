// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
use core::panic;

/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return          -1 if num is higher than the picked number
 *                  1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

struct Solution;
impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        let mut l = 1;
        let mut h = n;
        let mut pick = 0;
        while l <= h {
            let mid = l + (h - l) / 2;
            match Self::guess(mid) {
                0 => {
                    pick = mid;
                    break;
                }
                1 => h = mid - 1,
                -1 => l = mid + 1,
                _ => panic!("cant"),
            }
        }
        pick
    }

    unsafe fn guess(num: i32) -> i32 {
        num
    }
}
