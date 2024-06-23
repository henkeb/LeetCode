// Solution has
// Time complexity: O(log(n))
// Space complexity: O(1)
//
// You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
//
// Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
//
// You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

struct Solution;
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut h = n;

        while l < h {
            let mid = l + (l + h) / 2;
            if self.is_bad_version(mid) {
                h = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }

    fn is_bad_version(&self, _: i32) -> bool {
        true
    }
}
