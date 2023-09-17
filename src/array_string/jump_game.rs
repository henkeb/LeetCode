// Solution has:
// time complexity O(n)
// space complexity O(n)
struct Solution {}

#[derive(Clone)]
enum Index {
    Good,
    Bad,
    Unknown,
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut memo: Vec<Index> = vec![Index::Unknown; nums.len()];
        memo[nums.len() - 1] = Index::Good;
        Solution::jump(&nums, 0, &mut memo)
    }

    fn jump(nums: &Vec<i32>, position: usize, memo: &mut Vec<Index>) -> bool {
        match memo[position] {
            Index::Good => return true,
            Index::Bad => return false,
            Index::Unknown => (), // If Unknown then keep going
        }

        let furthest_jump = std::cmp::min(position + nums[position] as usize, nums.len() - 1); // Get furthest position to jump to. If it is larger than vector size, then use vector size
        for new_jump in (position + 1..=furthest_jump).rev() {
            if Solution::jump(nums, new_jump, memo) {
                memo[new_jump] = Index::Good;
                return true;
            }
        }
        memo[position] = Index::Bad;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = vec![2, 3, 1, 1, 4];
        assert!(Solution::can_jump(input));
    }

    #[test]
    fn test2() {
        let input = vec![3, 2, 1, 0, 4];
        assert!(!Solution::can_jump(input));
    }

    #[test]
    fn test3() {
        let input = vec![2, 5, 0, 0];
        assert!(Solution::can_jump(input));
    }
}
