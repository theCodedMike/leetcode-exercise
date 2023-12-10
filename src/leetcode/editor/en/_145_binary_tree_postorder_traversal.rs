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
        //let mut res = vec![];
        //Self::recursion_impl(root, &mut res);
        //res

        //Self::iteration_impl_1(root)
        //Self::iteration_impl_2(root)
        //Self::iteration_impl_3(root)
        Self::iteration_impl_4(root)
    }

    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        match root {
            None => {}
            Some(curr) => {
                Self::recursion_impl(curr.borrow_mut().left.take(), res);
                Self::recursion_impl(curr.borrow_mut().right.take(), res);
                res.push(curr.borrow().val);
            }
        }
    }

    /// 这种写法可以作为前序遍历或后序遍历的通用写法
    fn iteration_impl_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![root];

            while let Some(curr) = stack.pop() {
                res.push(curr.borrow().val); // Root
                if let Some(left) = curr.borrow_mut().left.take() {
                    stack.push(left); // Left
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    stack.push(right); // Right
                }
            }
        }

        res.reverse(); // NRL -> LRN
        res
    }

    fn iteration_impl_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        let mut last_visited = None;
        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                let right = curr.borrow_mut().right.take();
                if right.is_some() && !right.eq(&last_visited) {
                    root = right;
                    stack.push(curr);
                } else {
                    res.push(curr.borrow().val);
                    last_visited = right;
                }
            }
        }

        res
    }

    fn iteration_impl_3(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        let mut last_visited = None;
        while root.is_some() || !stack.is_empty() {
            match root {
                Some(curr) => {
                    root = curr.borrow_mut().left.take();
                    stack.push(curr);
                }
                None => {
                    if let Some(curr) = stack.pop() {
                        let right = curr.borrow_mut().right.take();
                        if right.is_some() && !right.eq(&last_visited) {
                            root = right;
                            stack.push(curr);
                        } else {
                            res.push(curr.borrow().val);
                            last_visited = right;
                        }
                    }
                }
            }
        }

        res
    }

    /// 这种写法可以作为通用写法
    fn iteration_impl_4(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![Ok(root)];

            while let Some(curr) = stack.pop() {
                match curr {
                    Ok(node) => {
                        stack.push(Err(node.borrow().val));
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push(Ok(right));
                        }
                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push(Ok(left));
                        }
                    }
                    Err(val) => {
                        res.push(val);
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
