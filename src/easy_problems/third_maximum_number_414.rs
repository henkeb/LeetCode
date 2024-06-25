// Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.
struct Solution;
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut top_three: [i64; 3] = [i64::MIN; 3];
        for num in nums {
            let num_i64: i64 = i64::from(num);
            match (
                num_i64 >= top_three[0],
                num_i64 >= top_three[1],
                num_i64 >= top_three[2],
            ) {
                (true, true, true) => {
                    if num_i64 != top_three[0] && num_i64 != top_three[1] && num_i64 != top_three[2]
                    {
                        top_three.swap(0, 1);
                        top_three.swap(0, 2);
                        top_three[0] = num_i64;
                    }
                }
                (false, true, true) => {
                    if num_i64 != top_three[1] && num_i64 != top_three[2] {
                        top_three.swap(1, 2);
                        top_three[1] = num_i64;
                    }
                }
                (false, false, true) => {
                    top_three[2] = num_i64;
                }
                (_, _, _) => (),
            }
        }
        match top_three.iter().filter(|&val| *val == i64::MIN).count() {
            0 => top_three[2] as i32,
            1 => top_three[0] as i32,
            2 => top_three[0] as i32,
            3 => top_three[0] as i32,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
    }
    #[test]
    fn two() {
        assert_eq!(Solution::third_max(vec![2, 1]), 2);
    }
    #[test]
    fn one() {
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
