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
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //Self::from_bottom_to_top(root)
        Self::from_top_to_bottom(root)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    ///
    fn from_bottom_to_top(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        const RECUR_HELPER: fn(Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) = |root| match root {
            None => (0, true),
            Some(curr) => {
                let (l_height, l_bal) = RECUR_HELPER(curr.borrow_mut().left.take());
                let (r_height, r_bal) = RECUR_HELPER(curr.borrow_mut().right.take());
                (
                    std::cmp::max(l_height, r_height) + 1,
                    l_bal && r_bal && (l_height - r_height).abs() <= 1,
                )
            }
        };

        RECUR_HELPER(root).1
    }

    ///
    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n)
    ///
    fn from_top_to_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        const CALC_HEIGHT: fn(Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| match root {
            None => 0,
            Some(curr) => {
                std::cmp::max(
                    CALC_HEIGHT(curr.borrow().left.clone()),
                    CALC_HEIGHT(curr.borrow().right.clone()),
                ) + 1
            }
        };

        const CHECK_BALANCE: fn(Option<Rc<RefCell<TreeNode>>>) -> bool = |root| match root {
            None => true,
            Some(curr) => {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                let l_height = CALC_HEIGHT(left.clone());
                let r_height = CALC_HEIGHT(right.clone());
                if (l_height - r_height).abs() > 1 {
                    return false;
                }

                CHECK_BALANCE(left) && CHECK_BALANCE(right)
            }
        };

        CHECK_BALANCE(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
