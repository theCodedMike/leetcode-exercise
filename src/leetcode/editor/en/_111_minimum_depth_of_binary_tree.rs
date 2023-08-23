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
    pub fn new2(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::recursion_helper(root)
        //Self::bfs_helper(root)
    }

    fn recursion_helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(curr) => {
                let left_child = curr.borrow_mut().left.take();
                let right_child = curr.borrow_mut().right.take();
                match (left_child, right_child) {
                    (None, None) => 1,
                    (l_child, r_child) => {
                        let mut min_depth = i32::MAX;
                        if l_child.is_some() {
                            min_depth = std::cmp::min(Self::recursion_helper(l_child), min_depth);
                        }
                        if r_child.is_some() {
                            min_depth = std::cmp::min(Self::recursion_helper(r_child), min_depth);
                        }
                        min_depth + 1
                    }
                }
            }
        }
    }

    ///
    /// 层序遍历
    ///
    fn bfs_helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_depth = i32::MAX;
        match root {
            None => min_depth = 0,
            Some(curr) => {
                let mut queue = VecDeque::from([(curr, 1)]);

                while !queue.is_empty() {
                    if let Some((curr, level)) = queue.pop_front() {
                        let l_child = curr.borrow_mut().left.take();
                        let r_child = curr.borrow_mut().right.take();

                        match (l_child, r_child) {
                            (None, None) => {
                                return level;
                            }
                            (left, right) => {
                                if let Some(left) = left {
                                    queue.push_back((left, level + 1));
                                }
                                if let Some(right) = right {
                                    queue.push_back((right, level + 1));
                                }
                            }
                        }
                    }
                }
            }
        }

        min_depth
    }
}
//leetcode submit region end(Prohibit modification and deletion)
