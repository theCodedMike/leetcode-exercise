//Given the roots of two binary trees p and q, write a function to check if
//they are the same or not.
//
// Two binary trees are considered the same if they are structurally identical,
//and the nodes have the same value.
//
//
// Example 1:
//
//
//Input: p = [1,2,3], q = [1,2,3]
//Output: true
//
//
// Example 2:
//
//
//Input: p = [1,2], q = [1,null,2]
//Output: false
//
//
// Example 3:
//
//
//Input: p = [1,2,1], q = [1,1,2]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in both trees is in the range [0, 100].
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 95
//90 👎 192

#![allow(dead_code)]
#![allow(unused_variables)]

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
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
