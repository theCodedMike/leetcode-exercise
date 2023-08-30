//Given the root of a binary tree, return the preorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [1,2,3]
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
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
//
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
//
// Related Topics Stack Tree Depth-First Search Binary Tree 👍 7384 👎 192

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //let mut res = vec![];
        //Self::recursion_helper(root, &mut res);
        //res
        Self::iteration_helper(root)
    }
    pub fn recursion_helper(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => {}
            Some(curr) => {
                let val = curr.borrow().val;
                res.push(val);
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                Self::recursion_helper(left, res);
                Self::recursion_helper(right, res);
            }
        }
    }

    pub fn iteration_helper(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                let val = curr.borrow().val;
                res.push(val);
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                root = curr.borrow_mut().right.take();
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
