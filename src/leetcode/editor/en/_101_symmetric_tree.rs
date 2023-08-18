//Given the root of a binary tree, check whether it is a mirror of itself (i.e.,
// symmetric around its center).
//
//
// Example 1:
//
//
//Input: root = [1,2,2,3,4,4,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [1,2,2,null,3,null,3]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Could you solve it both recursively and iteratively?
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 14
//140 👎 322

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //Self::recursion_helper(root.clone(), root)
        //Self::iteration_queue_helper(root)
        Self::iteration_stack_helper(root)
    }

    ///
    /// 递归
    ///
    fn recursion_helper(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val
                    && Self::recursion_helper(l.borrow().left.clone(), r.borrow().right.clone())
                    && Self::recursion_helper(l.borrow().right.clone(), r.borrow().left.clone())
            }
            _ => false,
        }
    }

    ///
    /// 迭代
    ///
    /// 层次遍历
    fn iteration_queue_helper(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let mut queue = VecDeque::new();
                let mut ref_mut = root.borrow_mut();
                queue.push_back((ref_mut.left.take(), ref_mut.right.take()));

                while !queue.is_empty() {
                    if let Some(pair) = queue.pop_front() {
                        match pair {
                            (None, None) => continue,
                            (Some(l), Some(r)) => {
                                let mut l_ref = l.borrow_mut();
                                let mut r_ref = r.borrow_mut();
                                if l_ref.val != r_ref.val {
                                    return false;
                                }

                                queue.push_back((l_ref.left.take(), r_ref.right.take()));
                                queue.push_back((l_ref.right.take(), r_ref.left.take()));
                            }
                            _ => return false,
                        }
                    }
                }

                true
            }
        }
    }

    ///
    /// 栈迭代
    ///
    /// 中序遍历
    fn iteration_stack_helper(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => {
                let mut ref_mut = root.borrow_mut();
                let mut l = ref_mut.left.take();
                let mut r = ref_mut.right.take();
                let mut l_stack = vec![];
                let mut r_stack = vec![];

                while l.is_some() || !l_stack.is_empty() {
                    while let Some(ln) = l {
                        l = ln.borrow_mut().left.take();
                        l_stack.push(ln);
                        match r {
                            None => return false,
                            Some(rn) => {
                                r = rn.borrow_mut().right.take();
                                r_stack.push(rn);
                            }
                        }
                    }

                    if r.is_some() {
                        return false;
                    }

                    match (l_stack.pop(), r_stack.pop()) {
                        (None, None) => continue,
                        (Some(ln), Some(rn)) => {
                            if ln.borrow().val != rn.borrow().val {
                                return false;
                            }
                            l = ln.borrow_mut().right.take();
                            r = rn.borrow_mut().left.take();
                        }
                        _ => return false,
                    }
                }

                if r.is_some() || !r_stack.is_empty() {
                    return false;
                }

                true
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
