//You are given the root of a binary search tree (BST) and an integer val.
//
// Find the node in the BST that the node's value equals val and return the
//subtree rooted with that node. If such a node does not exist, return null.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3], val = 2
//Output: [2,1,3]
//
//
// Example 2:
//
//
//Input: root = [4,2,7,1,3], val = 5
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 5000].
// 1 <= Node.val <= 10⁷
// root is a binary search tree.
// 1 <= val <= 10⁷
//
//
// Related Topics Tree Binary Search Tree Binary Tree 👍 5723 👎 181

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::recursion(root, val)
        //Self::iteration(root, val)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    ///
    fn recursion(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        const SEARCH: fn(Option<Rc<RefCell<TreeNode>>>, i32) -> Option<Rc<RefCell<TreeNode>>> =
            |root, val| match root {
                None => None,
                Some(curr) => {
                    let curr_val = curr.borrow().val;
                    let left = curr.borrow().left.clone();
                    let right = curr.borrow().right.clone();

                    if val > curr_val {
                        SEARCH(right, val)
                    } else if val < curr_val {
                        SEARCH(left, val)
                    } else {
                        Some(curr)
                    }
                }
            };

        SEARCH(root, val)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    ///
    fn iteration(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(curr) = root {
            let curr_val = curr.borrow().val;
            let left = curr.borrow().left.clone();
            let right = curr.borrow().right.clone();

            if val > curr_val {
                root = right;
            } else if val < curr_val {
                root = left;
            } else {
                return Some(curr);
            }
        }

        None
    }
}
//leetcode submit region end(Prohibit modification and deletion)
