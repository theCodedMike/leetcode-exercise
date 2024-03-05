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
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Self::recursion_impl(root)

        //Self::iteration_impl_1(root)
        //Self::iteration_impl_2(root)
        //Self::iteration_impl_3(root)
        //Self::iteration_impl_4(root)

        Self::morris_impl(root)
    }

    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        const POSTORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) = |root, res| {
            if let Some(curr) = root {
                POSTORDER(curr.borrow_mut().left.take(), res);
                POSTORDER(curr.borrow_mut().right.take(), res);

                res.push(curr.borrow().val);
            }
        };

        POSTORDER(root, &mut res);

        res
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

    fn morris_impl(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut add_path = |mut curr_left: Option<Rc<RefCell<TreeNode>>>| {
            let mut vals = vec![];

            while let Some(curr) = curr_left {
                vals.push(curr.borrow().val);
                curr_left = curr.borrow().right.clone();
            }

            vals.into_iter().rev().for_each(|val| res.push(val));
        };

        let mut root_node = root.clone();
        while let Some(curr) = root_node {
            let left = curr.borrow().left.clone();

            if left.is_some() {
                let mut prev_node = left.clone();
                while let Some(ref prev) = prev_node {
                    let right = prev.borrow().right.clone();
                    if right.is_none() || right == Some(curr.clone()) {
                        break;
                    } else {
                        prev_node = right;
                    }
                }

                match prev_node {
                    None => break,
                    Some(prev) => {
                        let right = prev.borrow().right.clone();
                        if right.is_some() {
                            prev.borrow_mut().right.take();
                            add_path(left);
                            root_node = curr.borrow().right.clone();
                        } else {
                            prev.borrow_mut().right = Some(curr);
                            root_node = left;
                        }
                    }
                }
            } else {
                root_node = curr.borrow().right.clone();
            }
        }

        add_path(root);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
