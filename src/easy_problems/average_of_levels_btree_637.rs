// Solution has
// Time complexity: O(n)
// Space complexity: O(1)
//
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        use std::collections::HashMap;
        fn dfs(
            node: &Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            levels: &mut HashMap<usize, (f64, usize)>,
        ) {
            if let Some(node) = node {
                let node = node.as_ref().borrow();
                levels
                    .entry(level)
                    .and_modify(|(sum, num)| {
                        *num += 1;
                        *sum += node.val as f64;
                    })
                    .or_insert((node.val as f64, 1));

                dfs(&node.left, level + 1, levels);
                dfs(&node.right, level + 1, levels);
            }
        }
        let mut levels: HashMap<usize, (f64, usize)> = HashMap::new();
        let level: usize = 0;
        dfs(&root, level, &mut levels);
        (0..levels.len())
            .map(|i| {
                let (key, value) = levels.get(&i).unwrap();
                key / *value as f64
            })
            .collect()
    }
}
