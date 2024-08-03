// Solution has
// Time complexity: O(n³)
// Space complexity: O(n²)
//
// There are n cities numbered from 0 to n-1. Given the array edges where edges[i] = [fromi, toi, weighti] represents a bidirectional and weighted edge between cities fromi and toi, and given the integer distanceThreshold.
//
// Return the city with the smallest number of cities that are reachable through some path and whose distance is at most distanceThreshold, If there are multiple such cities, return the city with the greatest number.
//
// Notice that the distance of a path connecting cities i and j is equal to the sum of the edges' weights along that path.
struct Solution;
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let nusize = n as usize;
        let mut distance_matrix = vec![vec![1e9 as i32 + 7; nusize]; nusize];
        (0..nusize).for_each(|i| {
            distance_matrix[i][i] = 0;
        });
        for edge in edges {
            let (src, dst, weight) = (edge[0] as usize, edge[1] as usize, edge[2]);
            distance_matrix[src][dst] = weight;
            distance_matrix[dst][src] = weight;
        }
        //Floyd-Warshall's algo
        for k in 0..nusize {
            for i in 0..nusize {
                for j in 0..nusize {
                    //i-j v.s. i-k-j, k is intermediate city
                    distance_matrix[i][j] =
                        distance_matrix[i][j].min(distance_matrix[i][k] + distance_matrix[k][j]);
                }
            }
        }
        let mut ans = 0; // the greatest number city with fewest reachable cities counts
        let mut fewest_reachable_cnt = n;
        for i in 0..n {
            let mut reachable_cnt = 0;
            for j in 0..n {
                if i != j && distance_matrix[i as usize][j as usize] <= distance_threshold {
                    reachable_cnt += 1;
                }
            }
            if reachable_cnt <= fewest_reachable_cnt {
                fewest_reachable_cnt = reachable_cnt;
                ans = i;
            }
        }
        ans
    }
}
