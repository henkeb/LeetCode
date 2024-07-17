// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// The Leetcode file system keeps a log each time some user performs a change folder operation.
//
// The operations are described below:
//
//     "../" : Move to the parent folder of the current folder. (If you are already in the main folder, remain in the same folder).
//     "./" : Remain in the same folder.
//     "x/" : Move to the child folder named x (This folder is guaranteed to always exist).
//
// You are given a list of strings logs where logs[i] is the operation performed by the user at the ith step.
//
// The file system starts in the main folder, then the operations in logs are performed.
//
// Return the minimum number of operations needed to go back to the main folder after the change folder operations.
struct Solution;
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut folder_level = 0;

        for operation in logs {
            match operation.as_str() {
                "./" => (),
                "../" => {
                    if folder_level > 0 {
                        folder_level -= 1;
                    }
                }
                _ => folder_level += 1,
            }
        }
        folder_level
    }
}
