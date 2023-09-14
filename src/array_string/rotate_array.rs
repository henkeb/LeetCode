struct Solution {}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let modulo = k as usize % nums.len();

        // Original List                   : 1 2 3 4 5 6 7
        // After reversing all numbers     : 7 6 5 4 3 2 1
        // After reversing first k numbers : 5 6 7 4 3 2 1
        // After revering last n-k numbers : 5 6 7 1 2 3 4 --> Result

        nums.reverse();
        nums[..modulo].reverse();
        nums[modulo..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut input:Vec<i32> = vec![1,2,3,4,5,6,7];
        let k = 3;
        Solution::rotate(&mut input, k);
        assert_eq!(vec![5,6,7,1,2,3,4], input);

    }

    #[test]
    fn test2() {
        let mut input:Vec<i32> = vec![-1,-100,3,99];
        let k = 2;
        Solution::rotate(&mut input, k);
        assert_eq!(vec![3,99,-1,-100], input);
    }
}
