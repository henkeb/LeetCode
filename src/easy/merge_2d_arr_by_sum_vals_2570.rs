// You are given two 2D integer arrays nums1 and nums2.
//
// nums1[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
// nums2[i] = [idi, vali] indicate that the number with the id idi has a value equal to vali.
// Each array contains unique ids and is sorted in ascending order by id.
//
// Merge the two arrays into one array that is sorted in ascending order by id, respecting the following conditions:
//
// Only ids that appear in at least one of the two arrays should be included in the resulting array.
// Each id should be included only once and its value should be the sum of the values of this id in the two arrays. If the id does not exist in one of the two arrays, then assume its value in that array to be 0.
// Return the resulting array. The returned array must be sorted in ascending order by id.
//
//
//
// Example 1:
//
// Input: nums1 = [[1,2],[2,3],[4,5]], nums2 = [[1,4],[3,2],[4,1]]
// Output: [[1,6],[2,3],[3,2],[4,6]]
// Explanation: The resulting array contains the following:
// - id = 1, the value of this id is 2 + 4 = 6.
// - id = 2, the value of this id is 3.
// - id = 3, the value of this id is 2.
// - id = 4, the value of this id is 5 + 1 = 6.
// Example 2:
//
// Input: nums1 = [[2,4],[3,6],[5,5]], nums2 = [[1,3],[4,3]]
// Output: [[1,3],[2,4],[3,6],[4,3],[5,5]]
// Explanation: There are no common ids, so we just include each id with its value in the resulting list.
//
//
// Constraints:
//
// 1 <= nums1.length, nums2.length <= 200
// nums1[i].length == nums2[j].length == 2
// 1 <= idi, vali <= 1000
// Both arrays contain unique ids.
// Both arrays are in strictly ascending order by id.
struct Solution;
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (mut nums1_idx, mut nums2_idx) = (0, 0);
        let (len1, len2) = (nums1.len(), nums2.len());
        let mut arr = Vec::new();
        while nums1_idx < len1 && nums2_idx < len2 {
            match nums1[nums1_idx][0].cmp(&nums2[nums2_idx][0]) {
                std::cmp::Ordering::Less => {
                    arr.push(nums1[nums1_idx].clone());
                    nums1_idx += 1;
                }
                std::cmp::Ordering::Equal => {
                    let mut val = nums1[nums1_idx].clone();
                    val[1] += nums2[nums2_idx][1];
                    arr.push(val);
                    nums1_idx += 1;
                    nums2_idx += 1;
                }
                std::cmp::Ordering::Greater => {
                    arr.push(nums2[nums2_idx].clone());
                    nums2_idx += 1;
                }
            }
        }
        for i in nums1_idx..len1 {
            arr.push(nums1[i].clone());
        }
        for i in nums2_idx..len2 {
            arr.push(nums2[i].clone());
        }
        arr
    }
}
