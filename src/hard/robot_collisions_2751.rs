// Solution has
// Time complexity: O(n*log(n))
// Space complexity: O(n)
//
// There are n 1-indexed robots, each having a position on a line, health, and movement direction.
//
// You are given 0-indexed integer arrays positions, healths, and a string directions (directions[i] is either 'L' for left or 'R' for right). All integers in positions are unique.
//
// All robots start moving on the line simultaneously at the same speed in their given directions. If two robots ever share the same position while moving, they will collide.
//
// If two robots collide, the robot with lower health is removed from the line, and the health of the other robot decreases by one. The surviving robot continues in the same direction it was going. If both robots have the same health, they are both removed from the line.
//
// Your task is to determine the health of the robots that survive the collisions, in the same order that the robots were given, i.e. final heath of robot 1 (if survived), final health of robot 2 (if survived), and so on. If there are no survivors, return an empty array.
//
// Return an array containing the health of the remaining robots (in the order they were given in the input), after no further collisions can occur.
//
// Note: The positions may be unsorted.

struct Solution;
impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        mut healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots = (0..positions.len()).collect::<Vec<usize>>();
        let directions = directions.as_bytes();
        robots.sort_unstable_by_key(|i| positions[*i]);

        let mut movement_stack = Vec::new();

        for robot in robots.iter_mut() {
            if directions[*robot] == b'R' {
                movement_stack.push(robot);
            } else {
                while let Some(colliding_robot) = movement_stack.pop() {
                    use std::cmp::Ordering;
                    match healths[*robot].cmp(&healths[*colliding_robot]) {
                        Ordering::Greater => {
                            healths[*colliding_robot] = 0;
                            healths[*robot] -= 1;
                            if healths[*robot] == 0 {
                                break;
                            }
                        }
                        Ordering::Equal => {
                            healths[*colliding_robot] = 0;
                            healths[*robot] = 0;
                            break;
                        }
                        Ordering::Less => {
                            healths[*robot] = 0;
                            healths[*colliding_robot] -= 1;
                            if healths[*colliding_robot] > 0 {
                                movement_stack.push(colliding_robot);
                            }
                            break;
                        }
                    }
                }
            }
        }

        robots.sort_unstable();
        robots
            .into_iter()
            .filter(|&robot| healths[robot] != 0)
            .map(|robot| healths[robot])
            .collect()
    }
}
