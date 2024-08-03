// Solution has
// Time complexity: O(nlogn)
// Space complexity: O(1)
//
// Given an array of integers nums, sort the array in ascending order and return it.
//
// You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
struct Solution;
impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::build_max_heap(&mut nums);

        let mut end = nums.len() - 1;
        while end > 0 {
            nums.swap(0, end);
            Self::max_heapify(&mut nums[..end], 0);
            end -= 1;
        }
        nums
    }

    fn max_heapify(nums: &mut [i32], i: usize) {
        let mut largest = i;
        let l = i * 2 + 1;
        let r = i * 2 + 2;
        if l < nums.len() && nums[l] > nums[largest] {
            largest = l;
        }
        if r < nums.len() && nums[r] > nums[largest] {
            largest = r;
        }
        if largest != i {
            nums.swap(i, largest);
            Self::max_heapify(nums, largest);
        }
    }
    fn build_max_heap(nums: &mut [i32]) {
        let mut i = (nums.len() - 1) / 2;
        while i > 0 {
            Self::max_heapify(nums, i);
            i -= 1;
        }
        Self::max_heapify(nums, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algo_book() {
        let mut res = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        Solution::build_max_heap(&mut res);
        assert_eq!(res, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
    }

    #[test]
    fn test_sort_array() {
        assert_eq!(Solution::sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
        assert_eq!(
            Solution::sort_array(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }
}
