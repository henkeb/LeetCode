// Solution has
// Time complexity: O(n)
// Space complexity: O(obstacles.len)
//
// A robot on an infinite XY-plane starts at point (0, 0) facing north. The robot can receive a sequence of these three possible types of commands:
//
//     -2: Turn left 90 degrees.
//     -1: Turn right 90 degrees.
//     1 <= k <= 9: Move forward k units, one unit at a time.
//
// Some of the grid squares are obstacles. The ith obstacle is at grid point obstacles[i] = (xi, yi). If the robot runs into an obstacle, then it will instead stay in its current location and move on to the next command.
//
// Return the maximum Euclidean distance that the robot ever gets from the origin squared (i.e. if the distance is 5, return 25).
//
// Note:
//
//     North means +Y direction.
//     East means +X direction.
//     South means -Y direction.
//     West means -X direction.
//     There can be obstacle in [0,0].
//
//
//
// Example 1:
//
// Input: commands = [4,-1,3], obstacles = []
// Output: 25
// Explanation: The robot starts at (0, 0):
// 1. Move north 4 units to (0, 4).
// 2. Turn right.
// 3. Move east 3 units to (3, 4).
// The furthest point the robot ever gets from the origin is (3, 4), which squared is 32 + 42 = 25 units away.
//
// Example 2:
//
// Input: commands = [4,-1,4,-2,4], obstacles = [[2,4]]
// Output: 65
// Explanation: The robot starts at (0, 0):
// 1. Move north 4 units to (0, 4).
// 2. Turn right.
// 3. Move east 1 unit and get blocked by the obstacle at (2, 4), robot is at (1, 4).
// 4. Turn left.
// 5. Move north 4 units to (1, 8).
// The furthest point the robot ever gets from the origin is (1, 8), which squared is 12 + 82 = 65 units away.
//
// Example 3:
//
// Input: commands = [6,-1,-1,6], obstacles = []
// Output: 36
// Explanation: The robot starts at (0, 0):
// 1. Move north 6 units to (0, 6).
// 2. Turn right.
// 3. Turn right.
// 4. Move south 6 units to (0, 0).
// The furthest point the robot ever gets from the origin is (0, 6), which squared is 62 = 36 units away.
//
//
//
// Constraints:
//
//     1 <= commands.length <= 104
//     commands[i] is either -2, -1, or an integer in the range [1, 9].
//     0 <= obstacles.length <= 104
//     -3 * 104 <= xi, yi <= 3 * 104
//     The answer is guaranteed to be less than 231.
//
use std::collections::HashSet;

struct Solution;
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn value(&self) -> Coordinate {
        match *self {
            Self::North => Coordinate { x: 0, y: 1 },
            Self::East => Coordinate { x: 1, y: 0 },
            Self::South => Coordinate { x: 0, y: -1 },
            Self::West => Coordinate { x: -1, y: 0 },
        }
    }
}
impl Into<(i32, i32)> for Coordinate {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}
impl From<&Vec<i32>> for Coordinate {
    fn from(value: &Vec<i32>) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
    pub x: i32,
    pub y: i32,
}
impl Coordinate {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}
struct Robot {
    direction: Direction,
    position: Coordinate,
    max_distance: i32,
    obstacles: HashSet<Coordinate>,
}
impl Robot {
    fn new(obstacles: HashSet<Coordinate>) -> Self {
        Self {
            direction: Direction::North,
            position: Coordinate::new(),
            max_distance: 0,
            obstacles,
        }
    }

    fn rotate(&mut self, rotation: i32) {
        if rotation == -1 {
            self.direction = match self.direction {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            }
        } else {
            self.direction = match self.direction {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
            }
        }
    }

    fn walk(&mut self, distance: i32) {
        let (x_dir, y_dir): (i32, i32) = self.direction.value().into();
        for _ in 0..distance {
            match self.obstacles.get(&Coordinate {
                x: self.position.x + x_dir,
                y: self.position.y + y_dir,
            }) {
                Some(_) => break,
                None => {
                    self.position.x += x_dir;
                    self.position.y += y_dir;
                    self.update_max_distance();
                }
            }
        }
    }

    fn update_max_distance(&mut self) {
        self.max_distance = std::cmp::max(self.max_distance, self.calculate_euclidean_distance());
    }

    fn calculate_euclidean_distance(&self) -> i32 {
        self.position.x.pow(2) + self.position.y.pow(2)
    }
}
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles.iter().map(|v| v.into()).collect();
        let mut robot = Robot::new(obstacles);
        for &command in commands.iter() {
            match command {
                rot if rot == -1 || rot == -2 => robot.rotate(rot),
                step => robot.walk(step),
            }
        }
        robot.max_distance
    }
}
