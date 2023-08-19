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

    pub fn new2(val: i32, left: TreeNode, right: TreeNode) -> Self {
        TreeNode {
            val,
            left: Some(Rc::new(RefCell::new(left))),
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::recursion_preorder(root)
        Self::iteration_level_order(root)
    }

    fn recursion_preorder(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(curr) => {
                let mut ref_mut = curr.borrow_mut();
                std::cmp::max(
                    Self::recursion_preorder(ref_mut.left.take()),
                    Self::recursion_preorder(ref_mut.right.take()),
                ) + 1
            }
        }
    }

    fn iteration_level_order(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;

        match node {
            None => {}
            Some(curr) => {
                let mut queue = VecDeque::new();
                queue.push_back((curr, 1));

                while !queue.is_empty() {
                    if let Some((curr, level)) = queue.pop_front() {
                        depth = level;

                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back((left, level + 1));
                        }

                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back((right, level + 1));
                        }
                    }
                }
            }
        }

        depth
    }
}
//leetcode submit region end(Prohibit modification and deletion)
