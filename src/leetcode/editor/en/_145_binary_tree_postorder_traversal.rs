//Given the root of a binary tree, return the postorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [3,2,1]
//
//
// Example 2:
//
//
//Input: root = []
//Output: []
//
//
// Example 3:
//
//
//Input: root = [1]
//Output: [1]
//
//
//
// Constraints:
//
//
// The number of the nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Recursive solution is trivial, could you do it iteratively?
//
// Related Topics Stack Tree Depth-First Search Binary Tree 👍 6391 👎 181

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::iteration_helper(root)
        // let mut vals = vec![];
        // Self::recursion_helper(root, &mut vals);
        // vals
    }

    pub fn recursion_helper(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        match root {
            None => {}
            Some(curr) => {
                let val = curr.borrow().val;
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                Self::recursion_helper(left, vals);
                Self::recursion_helper(right, vals);
                vals.push(val);
            }
        }
    }

    pub fn iteration_helper(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vals = vec![];
        let mut stack = vec![];

        let mut prev = None;
        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                if curr.borrow().right.is_none() || curr.borrow().right.eq(&prev) {
                    vals.push(curr.borrow().val);
                    prev = Some(curr);
                    root = None;
                } else {
                    root = curr.borrow_mut().right.take();
                    stack.push(curr);
                }
            }
        }

        vals
    }
}
//leetcode submit region end(Prohibit modification and deletion)
