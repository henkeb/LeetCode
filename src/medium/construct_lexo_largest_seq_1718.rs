// Given an integer n, find a sequence that satisfies all of the following:
//
// The integer 1 occurs once in the sequence.
// Each integer between 2 and n occurs twice in the sequence.
// For every integer i between 2 and n, the distance between the two occurrences of i is exactly i.
// The distance between two numbers on the sequence, a[i] and a[j], is the absolute difference of their indices, |j - i|.
//
// Return the lexicographically largest sequence. It is guaranteed that under the given constraints, there is always a solution.
//
// A sequence a is lexicographically larger than a sequence b (of the same length) if in the first position where a and b differ, sequence a has a number greater than the corresponding number in b. For example, [0,1,9,0] is lexicographically larger than [0,1,5,6] because the first position they differ is at the third number, and 9 is greater than 5.
//
//
//
// Example 1:
//
// Input: n = 3
// Output: [3,1,2,3,2]
// Explanation: [2,3,2,1,3] is also a valid sequence, but [3,1,2,3,2] is the lexicographically largest valid sequence.
// Example 2:
//
// Input: n = 5
// Output: [5,3,1,4,3,5,2,4,2]
//
//
// Constraints:
//
// 1 <= n <= 20
struct Solution;
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut best_sequence: Vec<i32> = vec![0; n * 2 - 1];
        let mut init: Vec<i32> = vec![1; n * 2 - 1];
        init[0] = n as i32;
        init[n] = n as i32;
        let mut queue: Vec<(Vec<i32>, i32)> = Vec::new();
        queue.push((init, (n - 1) as i32));

        'outer: while let Some((sequence, level)) = queue.pop() {
            println!("queue: {queue:?}");
            if level == 1 {
                if sequence.iter().filter(|&val| *val == 1).count() == 1 {
                    for i in 1..best_sequence.len() {
                        match sequence[i].cmp(&best_sequence[i]) {
                            std::cmp::Ordering::Less => continue 'outer,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => {
                                best_sequence = sequence;
                                continue 'outer;
                            }
                        }
                    }
                }
                continue;
            }
            for i in 1..sequence.len() - level as usize {
                let mut new_sequence = sequence.clone();
                if new_sequence[i] == 1 && new_sequence[i + level as usize] == 1 {
                    new_sequence[i] = level;
                    new_sequence[i + level as usize] = level;
                    queue.push((new_sequence, level - 1));
                }
            }
        }
        best_sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::construct_distanced_sequence(3),
            vec![3, 1, 2, 3, 2]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        );
    }

    // #[test]
    // fn test3() {
    //     assert_eq!(
    //         Solution::construct_distanced_sequence(11),
    //         vec![11, 9, 10, 6, 4, 1, 7, 8, 4, 6, 9, 11, 10, 7, 5, 8, 2, 3, 2, 5, 3]
    //     );
    // }
}
