//Given the root of a binary tree, determine if it is a valid binary search
//tree (BST).
//
// A valid BST is defined as follows:
//
//
// The left subtree of a node contains only nodes with keys less than the
//node's key.
// The right subtree of a node contains only nodes with keys greater than the
//node's key.
// Both the left and right subtrees must also be binary search trees.
//
//
//
// Example 1:
//
//
//Input: root = [2,1,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [5,1,4,null,null,3,6]
//Output: false
//Explanation: The root node's value is 5 but its right child's value is 4.
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
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 1545
//0 👎 1256

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
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let use_recursive = false;
        if use_recursive {
            Solution::recursive_helper(root, i64::MIN, i64::MAX)
        } else {
            Solution::loop_helper(root)
        }
    }

    fn recursive_helper(node: Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match node {
            None => true,
            Some(curr) => {
                let mut ref_mut = curr.borrow_mut();
                let curr_val = ref_mut.val as i64;
                if curr_val <= min || curr_val >= max {
                    return false;
                }
                return Solution::recursive_helper(ref_mut.left.take(), min, curr_val)
                    && Solution::recursive_helper(ref_mut.right.take(), curr_val, max);
            }
        }
    }

    fn loop_helper(mut node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev_val: i64 = i64::MIN;
        let mut stack = vec![];

        while node.is_some() || !stack.is_empty() {
            while let Some(curr) = node.take() {
                node = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                let curr_val = curr.borrow_mut().val as i64;
                if prev_val >= curr_val {
                    return false;
                }
                prev_val = curr_val;
                node = curr.borrow_mut().right.take();
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
