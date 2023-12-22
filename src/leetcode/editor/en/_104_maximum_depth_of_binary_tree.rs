//Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path
//from the root node down to the farthest leaf node.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: 3
//
//
// Example 2:
//
//
//Input: root = [1,null,2]
//Output: 2
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 10⁴].
// -100 <= Node.val <= 100
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 11
//586 👎 190

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dfs_recur(root)
        //Self::bfs_iter_1(root)
        Self::bfs_iter_2(root)
    }
    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const HELPER: fn(Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| {
            if let Some(curr) = root {
                std::cmp::max(
                    HELPER(curr.borrow().left.clone()),
                    HELPER(curr.borrow().right.clone()),
                ) + 1
            } else {
                0
            }
        };

        HELPER(root)
    }

    fn bfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 1)]);

            while let Some((curr, level)) = queue.pop_front() {
                max_depth = level;

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, level + 1));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, level + 1));
                }
            }
        }

        max_depth
    }

    fn bfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);
            while !queue.is_empty() {
                let level_len = queue.len();

                for _ in 0..level_len {
                    if let Some(curr) = queue.pop_front() {
                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }

                max_depth += 1;
            }
        }

        max_depth
    }
}
//leetcode submit region end(Prohibit modification and deletion)
