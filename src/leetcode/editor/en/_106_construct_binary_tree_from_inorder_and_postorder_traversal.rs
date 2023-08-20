//Given two integer arrays inorder and postorder where inorder is the inorder
//traversal of a binary tree and postorder is the postorder traversal of the same
//tree, construct and return the binary tree.
//
//
// Example 1:
//
//
//Input: inorder = [9,3,15,20,7], postorder = [9,15,7,20,3]
//Output: [3,9,20,null,null,15,7]
//
//
// Example 2:
//
//
//Input: inorder = [-1], postorder = [-1]
//Output: [-1]
//
//
//
// Constraints:
//
//
// 1 <= inorder.length <= 3000
// postorder.length == inorder.length
// -3000 <= inorder[i], postorder[i] <= 3000
// inorder and postorder consist of unique values.
// Each value of postorder also appears in inorder.
// inorder is guaranteed to be the inorder traversal of the tree.
// postorder is guaranteed to be the postorder traversal of the tree.
//
//
// Related Topics Array Hash Table Divide and Conquer Tree Binary Tree 👍 7455 ?
//? 114

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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recursion_helper(&inorder, &postorder)
        Self::iteration_helper(inorder, postorder)
    }

    fn recursion_helper(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        if postorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(postorder[0]))));
        }

        let (&root_val, postorder) = postorder.split_last().unwrap();
        let childs = inorder
            .split(|v| *v == root_val)
            .map(|sub| (sub, sub.len()))
            .collect::<Vec<_>>();
        let (left_inorder, l_len) = childs[0];
        let (right_inorder, _r_len) = childs[1];
        let (left_postorder, right_postorder) = postorder.split_at(l_len);

        let mut root = TreeNode::new(root_val);
        root.left = Self::recursion_helper(left_inorder, left_postorder);
        root.right = Self::recursion_helper(right_inorder, right_postorder);

        Some(Rc::new(RefCell::new(root)))
    }

    fn iteration_helper(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let last_post_idx = postorder.len() - 1;
        let root = Rc::new(RefCell::new(TreeNode::new(postorder[last_post_idx])));
        let mut curr = root.clone();
        let mut stack = vec![root.clone()];
        let mut in_idx = inorder.len() - 1;

        for post_idx in (0..last_post_idx).rev() {
            let postorder_val = postorder[post_idx];

            if curr.borrow().val != inorder[in_idx] {
                let right = Rc::new(RefCell::new(TreeNode::new(postorder_val)));
                curr.borrow_mut().right = Some(right.clone());
                stack.push(right.clone());
                curr = right;
            } else {
                while let Some(last) = stack.last() {
                    if last.borrow().val == inorder[in_idx] {
                        curr = stack.pop().unwrap();
                        in_idx -= 1;
                    } else {
                        break;
                    }
                }
                let left = Rc::new(RefCell::new(TreeNode::new(postorder_val)));
                curr.borrow_mut().left = Some(left.clone());
                stack.push(left.clone());
                curr = left;
            }
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
