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
        //Self::recursion_impl(root, &mut res);
        //res

        Self::iteration_impl_1(root)
        //Self::iteration_impl_2(root)
        //Self::iteration_impl_3(root)
        //Self::iteration_impl_4(root)
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n), the space is taken up by the recursive call stack.
    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => {}
            Some(node) => {
                res.push(node.borrow().val);
                Self::recursion_impl(node.borrow_mut().left.take(), res);
                Self::recursion_impl(node.borrow_mut().right.take(), res);
            }
        }
    }

    fn iteration_impl_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(node) = root {
            let mut stack = vec![node];
            while !stack.is_empty() {
                if let Some(curr) = stack.pop() {
                    res.push(curr.borrow().val);

                    if let Some(right) = curr.borrow_mut().right.take() {
                        stack.push(right);
                    }
                    if let Some(left) = curr.borrow_mut().left.take() {
                        stack.push(left);
                    }
                }
            }
        }

        res
    }

    fn iteration_impl_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                res.push(curr.borrow().val);
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                root = curr.borrow_mut().right.take();
            }
        }

        res
    }

    fn iteration_impl_3(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            match root {
                Some(curr) => {
                    res.push(curr.borrow().val);
                    root = curr.borrow_mut().left.take();
                    stack.push(curr);
                }
                None => {
                    if let Some(curr) = stack.pop() {
                        root = curr.borrow_mut().right.take();
                    }
                }
            }
        }

        res
    }

    fn iteration_impl_4(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if root.is_some() {
            let mut stack = vec![root];
            while !stack.is_empty() {
                match stack.pop().unwrap() {
                    Some(curr) => {
                        if let Some(right) = curr.borrow_mut().right.take() {
                            stack.push(Some(right)); // right
                        }
                        if let Some(left) = curr.borrow_mut().left.take() {
                            stack.push(Some(left)); // left
                        }
                        stack.push(Some(curr)); // root
                        stack.push(None);
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
