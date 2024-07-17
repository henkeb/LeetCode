// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
struct Solution;
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut three_largest = [i32::MIN; 3];
        let mut two_smallest = [i32::MAX; 2];
        for num in nums {
            if num >= three_largest[2] {
                three_largest = [three_largest[1], three_largest[2], num];
            } else if num >= three_largest[1] {
                three_largest = [three_largest[1], num, three_largest[2]];
            } else if num > three_largest[0] {
                three_largest = [num, three_largest[1], three_largest[2]];
            }
            if num <= two_smallest[0] {
                two_smallest = [num, two_smallest[0]];
            } else if num < two_smallest[1] {
                two_smallest = [two_smallest[0], num];
            }
        }

        std::cmp::max(
            three_largest.iter().product(),
            two_smallest.iter().product::<i32>() * three_largest[2],
        )
    }
}
