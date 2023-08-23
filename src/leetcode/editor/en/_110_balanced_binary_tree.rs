//Given a binary tree, determine if it is height-balanced.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [1,2,2,3,3,null,null,4,4]
//Output: false
//
//
// Example 3:
//
//
//Input: root = []
//Output: true
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 5000].
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics Tree Depth-First Search Binary Tree 👍 9690 👎 549

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
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::recursion_helper(root).1
    }

    /// from bottom to top
    fn recursion_helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        match root {
            None => (0, true),
            Some(curr) => {
                let (l_height, l_balance) = Self::recursion_helper(curr.borrow_mut().left.take());
                let (r_height, r_balance) = Self::recursion_helper(curr.borrow_mut().right.take());
                return (
                    std::cmp::max(l_height, r_height) + 1,
                    l_balance && r_balance && (l_height - r_height).abs() <= 1,
                );
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
