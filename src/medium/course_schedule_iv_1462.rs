use std::collections::{HashSet, VecDeque};

// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course ai first if you want to take course bi.
//
// For example, the pair [0, 1] indicates that you have to take course 0 before you can take course 1.
// Prerequisites can also be indirect. If course a is a prerequisite of course b, and course b is a prerequisite of course c, then course a is a prerequisite of course c.
//
// You are also given an array queries where queries[j] = [uj, vj]. For the jth query, you should answer whether course uj is a prerequisite of course vj or not.
//
// Return a boolean array answer, where answer[j] is the answer to the jth query.
//
//
//
// Example 1:
//
//
// Input: numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
// Output: [false,true]
// Explanation: The pair [1, 0] indicates that you have to take course 1 before you can take course 0.
// Course 0 is not a prerequisite of course 1, but the opposite is true.
// Example 2:
//
// Input: numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
// Output: [false,false]
// Explanation: There are no prerequisites, and each course is independent.
// Example 3:
//
//
// Input: numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
// Output: [true,true]
//
//
// Constraints:
//
// 2 <= numCourses <= 100
// 0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)
// prerequisites[i].length == 2
// 0 <= ai, bi <= numCourses - 1
// ai != bi
// All the pairs [ai, bi] are unique.
// The prerequisites graph has no cycles.
// 1 <= queries.length <= 104
// 0 <= ui, vi <= numCourses - 1
// ui != vi
//
// Solution has
// Time complexity: O(?)
// Space complexity: O(num_courses^2)
//
struct Solution;
impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut output = Vec::with_capacity(num_courses);
        let mut is_reachable: Vec<Vec<bool>> = vec![vec![false; num_courses]; num_courses];
        for pre_req in prerequisites.iter() {
            is_reachable[pre_req[1] as usize][pre_req[0] as usize] = true;
        }

        'outer: for query in queries.iter() {
            let mut queue: VecDeque<usize> = VecDeque::new();
            let target = query[0] as usize;
            queue.push_back(query[1] as usize);
            let mut visited: HashSet<usize> = HashSet::new();
            while let Some(node) = queue.pop_front() {
                let node = node as usize;
                if node == target {
                    output.push(true);
                    continue 'outer;
                }
                if visited.contains(&node) {
                    continue;
                }

                visited.insert(node);

                for i in 0..num_courses {
                    if i == node {
                        continue;
                    }
                    if is_reachable[node][i] {
                        queue.push_back(i);
                    }
                }
            }
            output.push(false);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::check_if_prerequisite(
                3,
                vec![vec![1, 2], vec![1, 0], vec![2, 0]],
                vec![vec![1, 0], vec![1, 2]]
            ),
            vec![true, true]
        )
    }
}
