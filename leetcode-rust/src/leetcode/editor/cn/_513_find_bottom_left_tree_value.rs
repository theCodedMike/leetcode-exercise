//Given the root of a binary tree, return the leftmost value in the last row of
//the tree.
//
//
// Example 1:
//
//
//Input: root = [2,1,3]
//Output: 1
//
//
// Example 2:
//
//
//Input: root = [1,2,3,4,null,5,6,null,null,7]
//Output: 7
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// -2³¹ <= Node.val <= 2³¹ - 1
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 31
//70 👎 257

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dfs_recur(root)
        //Self::dfs_iter(root)
        //Self::bfs_iter_1(root)
        Self::bfs_iter_2(root)
    }

    ///
    /// DFS - Recursion(Pre-Order)
    ///
    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut val = 0;
        let mut val_level = i32::MIN;
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, i32, &mut i32, &mut i32) =
            |root, level, val, val_level| {
                if let Some(curr) = root {
                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();
                    if left.is_none() && right.is_none() && level > *val_level {
                        *val = curr.borrow().val;
                        *val_level = level;
                    }

                    RECUR(left, level + 1, val, val_level);
                    RECUR(right, level + 1, val, val_level);
                }
            };

        RECUR(root, 0, &mut val, &mut val_level);

        val
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut val = 0;
        let mut val_level = i32::MIN;

        if let Some(root) = root {
            let mut stack = vec![(root, 0)];

            while let Some((curr, level)) = stack.pop() {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && level > val_level {
                    val = curr.borrow().val;
                    val_level = level;
                }

                if let Some(right) = right {
                    stack.push((right, level + 1));
                }
                if let Some(left) = left {
                    stack.push((left, level + 1));
                }
            }
        }

        val
    }

    ///
    /// BFS - Iteration
    ///
    fn bfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut val = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();

                for i in 0..level_len {
                    if let Some(curr) = queue.pop_front() {
                        if i == 0 {
                            val = curr.borrow().val;
                        }
                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }
            }
        }

        val
    }

    ///
    /// BFS - Iteration
    ///
    fn bfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut val = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);
            let mut prev_level = -1;

            while let Some((curr, level)) = queue.pop_front() {
                if prev_level != level {
                    val = curr.borrow().val;
                }
                prev_level = level;

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, level + 1));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, level + 1));
                }
            }
        }

        val
    }
}
//leetcode submit region end(Prohibit modification and deletion)
