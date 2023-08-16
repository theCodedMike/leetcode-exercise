//You are given the root of a binary search tree (BST), where the values of
//exactly two nodes of the tree were swapped by mistake. Recover the tree without
//changing its structure.
//
//
// Example 1:
//
//
//Input: root = [1,3,null,null,2]
//Output: [3,1,null,null,2]
//Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3
//makes the BST valid.
//
//
// Example 2:
//
//
//Input: root = [3,1,4,null,null,2]
//Output: [2,1,4,null,null,3]
//Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2
//and 3 makes the BST valid.
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [2, 1000].
// -2³¹ <= Node.val <= 2³¹ - 1
//
//
//
//Follow up: A solution using
//O(n) space is pretty straight-forward. Could you devise a constant
//O(1) space solution?
//
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 7348
// 👎 236

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
    pub fn new(val: i32, left: Option<TreeNode>, right: Option<TreeNode>) -> Self {
        TreeNode {
            val,
            left: left.map(|v| Rc::new(RefCell::new(v))),
            right: right.map(|v| Rc::new(RefCell::new(v))),
        }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut stack = vec![];
        let mut cur = root.clone();
        let mut left = None;
        let mut right = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        while !stack.is_empty() || cur.is_some() {
            while let Some(node) = cur {
                cur = node.borrow_mut().left.clone();
                stack.push(node);
            }

            if let Some(node) = stack.pop() {
                if let Some(p) = prev {
                    if p.borrow_mut().val > node.borrow_mut().val {
                        right = Some(node.clone());

                        if left.is_none() {
                            left = Some(p);
                        } else {
                            break;
                        }
                    }
                }

                prev = Some(node.clone());
                cur = node.borrow_mut().right.clone();
            }
        }

        std::mem::swap(
            &mut left.unwrap().borrow_mut().val,
            &mut right.unwrap().borrow_mut().val,
        )
    }
}
//leetcode submit region end(Prohibit modification and deletion)
