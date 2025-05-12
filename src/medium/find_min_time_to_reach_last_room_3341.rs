// There is a dungeon with n x m rooms arranged as a grid.
//
// You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes exactly one second.
//
// Return the minimum time to reach the room (n - 1, m - 1).
//
// Two rooms are adjacent if they share a common wall, either horizontally or vertically.
//
//
//
// Example 1:
//
// Input: moveTime = [[0,4],[4,4]]
//
// Output: 6
//
// Explanation:
//
// The minimum time required is 6 seconds.
//
// At time t == 4, move from room (0, 0) to room (1, 0) in one second.
// At time t == 5, move from room (1, 0) to room (1, 1) in one second.
// Example 2:
//
// Input: moveTime = [[0,0,0],[0,0,0]]
//
// Output: 3
//
// Explanation:
//
// The minimum time required is 3 seconds.
//
// At time t == 0, move from room (0, 0) to room (1, 0) in one second.
// At time t == 1, move from room (1, 0) to room (1, 1) in one second.
// At time t == 2, move from room (1, 1) to room (1, 2) in one second.
// Example 3:
//
// Input: moveTime = [[0,1],[1,2]]
//
// Output: 3
//
//
//
// Constraints:
//
// 2 <= n == moveTime.length <= 50
// 2 <= m == moveTime[i].length <= 50
// 0 <= moveTime[i][j] <= 109
struct Solution;
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let n = move_time.len();
        let m = move_time[0].len();
        let mut queue: Vec<((i32, i32), i32)> = Vec::new();
        let mut table: HashMap<(i32, i32), i32> = HashMap::new();
        queue.push(((0, 0), 0));

        while let Some(entry) = queue.pop() {
            let x = entry.0 .0;
            let y = entry.0 .1;
            let time = entry.1;
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let new_x = dx + x;
                let new_y = dy + y;
                if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 {
                    let mut new_time = time;
                    if new_time < move_time[new_y as usize][new_x as usize] {
                        new_time = move_time[new_y as usize][new_x as usize];
                    }
                    new_time += 1;
                    if let Some(prev) = table.get(&(new_x, new_y)) {
                        if *prev <= new_time {
                            continue;
                        }
                    }
                    table
                        .entry((new_x, new_y))
                        .and_modify(|val| *val = new_time)
                        .or_insert(new_time);
                    queue.push(((new_x, new_y), new_time));
                }
            }
        }
        *table.get(&(m as i32 - 1, n as i32 - 1)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::min_time_to_reach(vec![vec![0, 0, 0], vec![0, 0, 0]]),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::min_time_to_reach(vec![vec![0, 1], vec![1, 2]]), 3);
    }
}
