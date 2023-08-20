//Given two integer arrays preorder and inorder where preorder is the preorder
//traversal of a binary tree and inorder is the inorder traversal of the same tree,
// construct and return the binary tree.
//
//
// Example 1:
//
//
//Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//Output: [3,9,20,null,null,15,7]
//
//
// Example 2:
//
//
//Input: preorder = [-1], inorder = [-1]
//Output: [-1]
//
//
//
// Constraints:
//
//
// 1 <= preorder.length <= 3000
// inorder.length == preorder.length
// -3000 <= preorder[i], inorder[i] <= 3000
// preorder and inorder consist of unique values.
// Each value of inorder also appears in preorder.
// preorder is guaranteed to be the preorder traversal of the tree.
// inorder is guaranteed to be the inorder traversal of the tree.
//
//
// Related Topics Array Hash Table Divide and Conquer Tree Binary Tree 👍 13663
//👎 409

#![allow(dead_code)]
#![allow(unused)]

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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recursion_helper(&preorder, &inorder)
        Self::iteration_helper(preorder, inorder)
    }

    fn recursion_helper(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        // validate
        if preorder.len() == 0 {
            return None;
        }
        if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(preorder[0]))));
        }

        // prepare subtrees
        let (root_val, preorder) = preorder.split_first().unwrap();
        let childs = inorder
            .split(|v| *v == *root_val)
            .map(|child| (child, child.len()))
            .collect::<Vec<_>>();
        let (left_inorder, l_len) = childs[0];
        let (right_inorder, _r_len) = childs[1];
        let (left_preorder, right_preorder) = preorder.split_at(l_len);

        // create subtrees
        let left = Self::recursion_helper(left_preorder, left_inorder);
        let right = Self::recursion_helper(right_preorder, right_inorder);
        let mut root = TreeNode::new(*root_val);
        root.left = left;
        root.right = right;

        // create root node
        Some(Rc::new(RefCell::new(root)))
    }

    fn iteration_helper(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_idx = 0;
        let mut in_idx = 0;
        let mut root = Rc::new(RefCell::new(TreeNode::new(preorder[pre_idx])));
        pre_idx += 1;
        let mut curr = root.clone();
        let mut stack = vec![root.clone()];

        while pre_idx < preorder.len() {
            while curr.borrow().val != inorder[in_idx] {
                let left = Rc::new(RefCell::new(TreeNode::new(preorder[pre_idx])));
                pre_idx += 1;
                curr.borrow_mut().left = Some(left.clone());
                stack.push(left.clone());
                curr = left;
            }

            if pre_idx == preorder.len() {
                break;
            }

            while let Some(last) = stack.last() {
                if last.borrow().val == inorder[in_idx] {
                    curr = stack.pop().unwrap();
                    in_idx += 1;
                } else {
                    break;
                }
            }

            let right = Rc::new(RefCell::new(TreeNode::new(preorder[pre_idx])));
            pre_idx += 1;
            curr.borrow_mut().right = Some(right.clone());
            stack.push(right.clone());
            curr = right;
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
