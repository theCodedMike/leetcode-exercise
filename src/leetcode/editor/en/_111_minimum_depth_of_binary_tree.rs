//Given a binary tree, find its minimum depth.
//
// The minimum depth is the number of nodes along the shortest path from the
//root node down to the nearest leaf node.
//
// Note: A leaf is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: 2
//
//
// Example 2:
//
//
//Input: root = [2,null,3,null,4,null,5,null,6]
//Output: 5
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 10⁵].
// -1000 <= Node.val <= 1000
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 68
//08 👎 1227

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dfs_recur(root)
        //Self::dfs_pre_order_iter_1(root)
        //Self::dfs_pre_order_iter_2(root)
        //Self::dfs_pre_order_iter_3(root)
        Self::bfs_iter(root)
    }

    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const HELPER: fn(Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| {
            if let Some(curr) = root {
                match (curr.borrow().left.clone(), curr.borrow().right.clone()) {
                    (None, None) => 1,
                    (left, None) => HELPER(left) + 1,
                    (None, right) => HELPER(right) + 1,
                    (left, right) => std::cmp::min(HELPER(left), HELPER(right)) + 1,
                }
            } else {
                0
            }
        };

        HELPER(root)
    }

    fn dfs_pre_order_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut min_depth = i32::MAX;
        let mut stack = vec![];
        let mut root = (root, 1);

        while root.0.is_some() || !stack.is_empty() {
            while let Some(curr) = root.0 {
                let level = root.1;
                if curr.borrow().left.is_none()
                    && curr.borrow().right.is_none()
                    && level < min_depth
                {
                    min_depth = level;
                }

                root = (curr.borrow_mut().left.take(), level + 1);
                stack.push((curr, level));
            }

            if let Some((curr, level)) = stack.pop() {
                root = (curr.borrow_mut().right.take(), level + 1);
            }
        }

        min_depth
    }

    fn dfs_pre_order_iter_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut min_depth = i32::MAX;
        let mut stack = vec![];
        let mut root = (root, 1);
        while root.0.is_some() || !stack.is_empty() {
            if let Some(curr) = root.0 {
                let level = root.1;
                if curr.borrow().left.is_none()
                    && curr.borrow().right.is_none()
                    && level < min_depth
                {
                    min_depth = level;
                }

                root = (curr.borrow_mut().left.take(), level + 1);
                stack.push((curr, level));
            } else {
                if let Some((curr, level)) = stack.pop() {
                    root = (curr.borrow_mut().right.take(), level + 1);
                }
            }
        }

        min_depth
    }

    fn dfs_pre_order_iter_3(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_depth = 0;

        if let Some(root) = root {
            min_depth = i32::MAX;
            let mut stack = vec![(root, 1)];

            while let Some((curr, level)) = stack.pop() {
                if curr.borrow().left.is_none()
                    && curr.borrow().right.is_none()
                    && level < min_depth
                {
                    min_depth = level;
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    stack.push((right, level + 1));
                }
                if let Some(left) = curr.borrow_mut().left.take() {
                    stack.push((left, level + 1));
                }
            }
        }

        min_depth
    }

    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_depth = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 1)]);

            while !queue.is_empty() {
                if let Some((curr, level)) = queue.pop_front() {
                    if curr.borrow().left.is_none() && curr.borrow().right.is_none() {
                        min_depth = level;
                        break;
                    }
                    if let Some(left) = curr.borrow_mut().left.take() {
                        queue.push_back((left, level + 1));
                    }
                    if let Some(right) = curr.borrow_mut().right.take() {
                        queue.push_back((right, level + 1));
                    }
                }
            }
        }

        min_depth
    }
}
//leetcode submit region end(Prohibit modification and deletion)
