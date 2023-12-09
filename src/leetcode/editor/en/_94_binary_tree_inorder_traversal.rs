//Given the root of a binary tree, return the inorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [1,3,2]
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
//Follow up: Recursive solution is trivial, could you do it iteratively?
//
// Related Topics Stack Tree Depth-First Search Binary Tree 👍 11638 👎 595

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
    #[inline]
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //let mut res = vec![];
        //Self::recursion_impl(root, &mut res);
        //return res;

        //Self::iteration_impl_1(root)
        //Self::iteration_impl_2(root)
        Self::iteration_impl_3(root)
    }

    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => {}
            Some(root) => {
                Self::recursion_impl(root.borrow_mut().left.take(), res);
                res.push(root.borrow().val);
                Self::recursion_impl(root.borrow_mut().right.take(), res);
            }
        }
    }

    fn iteration_impl_1(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                res.push(curr.borrow().val);
                root = curr.borrow_mut().right.take();
            }
        }

        res
    }

    fn iteration_impl_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut stack = vec![];

        while root.is_some() || !stack.is_empty() {
            match root {
                Some(curr) => {
                    root = curr.borrow_mut().left.take();
                    stack.push(curr);
                }
                None => {
                    if let Some(curr) = stack.pop() {
                        res.push(curr.borrow().val);
                        root = curr.borrow_mut().right.take();
                    }
                }
            }
        }

        res
    }

    fn iteration_impl_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if root.is_some() {
            let mut stack = vec![root];
            while !stack.is_empty() {
                match stack.pop().unwrap() {
                    Some(curr) => {
                        let left = curr.borrow_mut().left.take();
                        let right = curr.borrow_mut().right.take();

                        if right.is_some() {
                            stack.push(right); // right
                        }
                        stack.push(Some(curr)); // root
                        stack.push(None);
                        if left.is_some() {
                            stack.push(left); // left
                        }
                    }
                    None => {
                        if let Some(curr) = stack.pop().unwrap() {
                            res.push(curr.borrow().val);
                        }
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
