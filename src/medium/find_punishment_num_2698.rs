use std::collections::HashMap;

// Given a positive integer n, return the punishment number of n.
//
// The punishment number of n is defined as the sum of the squares of all integers i such that:
//
// 1 <= i <= n
// The decimal representation of i * i can be partitioned into contiguous substrings such that the sum of the integer values of these substrings equals i.
//
//
// Example 1:
//
// Input: n = 10
// Output: 182
// Explanation: There are exactly 3 integers i in the range [1, 10] that satisfy the conditions in the statement:
// - 1 since 1 * 1 = 1
// - 9 since 9 * 9 = 81 and 81 can be partitioned into 8 and 1 with a sum equal to 8 + 1 == 9.
// - 10 since 10 * 10 = 100 and 100 can be partitioned into 10 and 0 with a sum equal to 10 + 0 == 10.
// Hence, the punishment number of 10 is 1 + 81 + 100 = 182
// Example 2:
//
// Input: n = 37
// Output: 1478
// Explanation: There are exactly 4 integers i in the range [1, 37] that satisfy the conditions in the statement:
// - 1 since 1 * 1 = 1.
// - 9 since 9 * 9 = 81 and 81 can be partitioned into 8 + 1.
// - 10 since 10 * 10 = 100 and 100 can be partitioned into 10 + 0.
// - 36 since 36 * 36 = 1296 and 1296 can be partitioned into 1 + 29 + 6.
// Hence, the punishment number of 37 is 1 + 81 + 100 + 1296 = 1478
//
//
// Constraints:
//
// 1 <= n <= 1000
struct Solution;
impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut output = 1;
        for i in 2..=n {
            let mut memo: HashMap<i32, usize> = HashMap::new();
            let square = Solution::get_num_in_chars(&(i * i));
            let mut queue: Vec<Vec<String>> = Vec::new();
            queue.push(square);
            while let Some(combination) = queue.pop() {
                let sum = Solution::add_numbers(&combination);
                if sum == i {
                    output += i * i;
                    break;
                }
                match memo.get(&sum) {
                    Some(val) => {
                        if *val == combination.len() {
                            continue;
                        }
                    }
                    None => (),
                }
                memo.insert(sum, combination.len());
                if combination.len() != 1 {
                    for i in 0..(combination.len() - 1) {
                        for j in i + 1..combination.len() {
                            let mut entry: Vec<String> = Vec::new();
                            let part = combination[i..=j].concat();

                            for i_interim in 0..i {
                                entry.push(combination[i_interim].clone());
                            }

                            entry.push(part);
                            for j_after in j + 1..combination.len() {
                                entry.push(combination[j_after].clone());
                            }
                            queue.push(entry);
                        }
                    }
                }
            }
        }
        output
    }

    fn get_num_in_chars(num: &i32) -> Vec<String> {
        let mut num = *num as u32;
        let mut output: Vec<String> = Vec::new();
        while num > 0 {
            output.push((num % 10).to_string());
            num /= 10;
        }
        output.reverse();
        output
    }

    fn add_numbers(nums: &Vec<String>) -> i32 {
        nums.iter().filter_map(|val| val.parse::<i32>().ok()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::punishment_number(37), 1478);
    }
    #[test]
    fn test2() {
        assert_eq!(Solution::punishment_number(72), 6528);
    }
}

// if combination.len() != 1 {
//     for i in 0..(combination.len() - 1) {
//         for j in 0..combination.len() {
//             if i == j {
//                 continue;
//             }
//             let mut entry: Vec<String> = Vec::new();
//             let mut s = combination[i].clone();
//             s.push_str(&combination[j]);
//             entry.push(s);
//             for k in 0..combination.len() {
//                 if k != i && k != j {
//                     entry.push(combination[k].clone());
//                 }
//             }
//             queue.push(entry);
//         }
//     }
// }
