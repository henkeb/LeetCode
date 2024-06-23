// Solution has
// Time complexity: O(n)
// Space complexity: O(n)
//
// You are given a sorted unique integer array nums.
//
// A range [a,b] is the set of all integers from a to b (inclusive).
//
// Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
//
// Each range [a,b] in the list should be output as:
//
//     "a->b" if a != b
//     "a" if a == b
struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();

        if nums.is_empty() {
            return output;
        }

        if nums.len() == 1 {
            output.push(format!("{}", nums[0]));
            return output;
        }

        let mut start = *nums.first().unwrap();
        let mut prev = *nums.first().unwrap();
        let mut count = 1;
        let len = nums.len();

        for num in nums.iter().skip(1) {
            count += 1;
            match num - prev {
                1 => {
                    prev = *num;
                    if count == len {
                        output.push(format!("{}->{}", start, prev));
                    }
                }
                _ => {
                    match start.cmp(&prev) {
                        std::cmp::Ordering::Equal => output.push(format!("{}", start)),
                        std::cmp::Ordering::Less => output.push(format!("{}->{}", start, prev)),
                        std::cmp::Ordering::Greater => panic!("Should not be able to enter here"),
                    }
                    start = *num;
                    prev = *num;
                    if count == len {
                        output.push(format!("{}", num));
                    }
                }
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::summary_ranges(vec![0]), vec!["0"]);
    }
}
