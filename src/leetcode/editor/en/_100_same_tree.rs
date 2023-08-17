//Given the roots of two binary trees p and q, write a function to check if
//they are the same or not.
//
// Two binary trees are considered the same if they are structurally identical,
//and the nodes have the same value.
//
//
// Example 1:
//
//
//Input: p = [1,2,3], q = [1,2,3]
//Output: true
//
//
// Example 2:
//
//
//Input: p = [1,2], q = [1,null,2]
//Output: false
//
//
// Example 3:
//
//
//Input: p = [1,2,1], q = [1,1,2]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in both trees is in the range [0, 100].
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 95
//90 👎 192

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
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let use_recursion = false;
        if use_recursion {
            Self::recursion_preorder_helper(p, q)
        } else {
            //Self::iteration_preorder_helper(p, q)
            Self::iteration_level_order_helper(p, q)
        }
    }

    fn recursion_preorder_helper(
        p_node: Option<Rc<RefCell<TreeNode>>>,
        q_node: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p_node, q_node) {
            (Some(p), Some(q)) => {
                let p_val = p.borrow().val;
                let q_val = q.borrow().val;
                if p_val != q_val {
                    return false;
                }

                Self::recursion_preorder_helper(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::recursion_preorder_helper(
                        p.borrow().right.clone(),
                        q.borrow().right.clone(),
                    )
            }
            (None, None) => true,
            _ => false,
        }
    }

    fn iteration_preorder_helper(
        mut p_node: Option<Rc<RefCell<TreeNode>>>,
        mut q_node: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![];

        while (p_node.is_some() && q_node.is_some()) || !stack.is_empty() {
            loop {
                match (p_node.clone(), q_node.clone()) {
                    (Some(p), Some(q)) => {
                        let p_val = p.borrow().val;
                        let q_val = q.borrow().val;
                        if p_val != q_val {
                            return false;
                        }
                        stack.push((p.clone(), q.clone()));
                        p_node = p.borrow().left.clone();
                        q_node = q.borrow().left.clone();
                    }
                    (None, None) => break,
                    _ => return false,
                }
            }

            if let Some((p, q)) = stack.pop() {
                p_node = p.borrow().right.clone();
                q_node = q.borrow().right.clone();
            }
        }

        p_node.is_none() && q_node.is_none()
    }

    fn iteration_level_order_helper(
        p_node: Option<Rc<RefCell<TreeNode>>>,
        q_node: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut queue = VecDeque::new();

        match (p_node, q_node) {
            (Some(p), Some(q)) => {
                queue.push_back((p, q));
            }
            (None, None) => return true,
            _ => return false,
        }

        while !queue.is_empty() {
            if let Some((p, q)) = queue.pop_front() {
                let p_val = p.borrow().val;
                let q_val = q.borrow().val;
                if p_val != q_val {
                    return false;
                }

                match (p.borrow().left.clone(), q.borrow().left.clone()) {
                    (Some(p_left), Some(q_left)) => {
                        queue.push_back((p_left, q_left));
                    }
                    (None, None) => {}
                    _ => return false,
                }

                match (p.borrow().right.clone(), q.borrow().right.clone()) {
                    (Some(p_right), Some(q_right)) => {
                        queue.push_back((p_right, q_right));
                    }
                    (None, None) => {}
                    _ => return false,
                }
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
