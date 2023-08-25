//Given the root of a binary tree, flatten the tree into a "linked list":
//
//
// The "linked list" should use the same TreeNode class where the right child
//pointer points to the next node in the list and the left child pointer is always
//null.
// The "linked list" should be in the same order as a pre-order traversal of
//the binary tree.
//
//
//
// Example 1:
//
//
//Input: root = [1,2,5,3,4,null,6]
//Output: [1,null,2,null,3,null,4,null,5,null,6]
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
//Input: root = [0]
//Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 2000].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Can you flatten the tree in-place (with
//O(1) extra space)?
//
// Related Topics Linked List Stack Tree Depth-First Search Binary Tree 👍 11250
// 👎 529

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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        //let _ = Self::recursion_helper(root);
        //Self::iteration_helper(root);
        Self::preorder_traversal_then_build(root);
        //Self::preorder_traversal_and_build(root);
    }

    /// 寻找前驱节点 - 递归
    pub fn recursion_helper(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(curr) => {
                let mut left = curr.borrow_mut().left.take();
                let mut right = curr.borrow_mut().right.take();

                match (left.clone(), right.clone()) {
                    (None, None) => None,
                    (None, Some(r_node)) => {
                        let r_return = Self::recursion_helper(&mut right);
                        match r_return {
                            None => {
                                // 2
                                //  \
                                //   3
                                curr.borrow_mut().right = Some(r_node.clone());
                                Some(r_node)
                            }
                            Some(r_r_node) => {
                                // 1
                                //  \
                                //   2
                                //    \
                                //     3
                                curr.borrow_mut().right = Some(r_node);
                                Some(r_r_node)
                            }
                        }
                    }
                    (Some(l_node), None) => {
                        let l_return = Self::recursion_helper(&mut left);
                        match l_return {
                            None => {
                                //   2
                                //  /
                                // 3
                                curr.borrow_mut().right = Some(l_node.clone());
                                Some(l_node)
                            }
                            Some(l_r_node) => {
                                //     1
                                //    /
                                //   2
                                //  /
                                // 3
                                curr.borrow_mut().right = Some(l_node);
                                Some(l_r_node)
                            }
                        }
                    }
                    (Some(l_node), Some(r_node)) => {
                        let l_return = Self::recursion_helper(&mut left);
                        let r_return = Self::recursion_helper(&mut right);

                        match (l_return, r_return) {
                            (None, None) => {
                                //    2
                                //   / \
                                //  3   4
                                l_node.borrow_mut().right = Some(r_node.clone());
                                curr.borrow_mut().right = Some(l_node);
                                Some(r_node)
                            }
                            (Some(l_r_node), None) => {
                                //      1
                                //     / \
                                //    2   5
                                //     \
                                //      3
                                l_r_node.borrow_mut().right = Some(r_node.clone());
                                curr.borrow_mut().right = Some(l_node);
                                Some(r_node)
                            }
                            (None, Some(r_r_node)) => {
                                //      1
                                //     / \
                                //    2   5
                                //         \
                                //          3
                                l_node.borrow_mut().right = Some(r_node);
                                curr.borrow_mut().right = Some(l_node);
                                Some(r_r_node)
                            }
                            (Some(l_r_node), Some(r_r_node)) => {
                                //       1
                                //     /   \
                                //    2     5
                                //     \     \
                                //      3     4
                                l_r_node.borrow_mut().right = Some(r_node);
                                curr.borrow_mut().right = Some(l_node);
                                Some(r_r_node)
                            }
                        }
                    }
                }
            }
        }
    }

    /// 寻找前驱节点 - 迭代
    pub fn iteration_helper(_root: &mut Option<Rc<RefCell<TreeNode>>>) {
        /*let mut curr_node = root.clone();
        while let Some(curr) = curr_node {
            let left_child = curr.borrow_mut().left.take();
            if let Some(left) = left_child.clone() {
                let mut predecessor = left.clone();
                while let Some(right) = predecessor.borrow().right.clone() {
                    // cannot assign to `predecessor` because it is borrowed
                    predecessor = right;
                }

                predecessor.borrow_mut().right = curr.borrow_mut().right.take();
                curr.borrow_mut().right = left_child;
            }

            curr_node = curr.borrow_mut().right.take();
        }*/
    }

    pub fn preorder_traversal_then_build(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut root_clone = root.clone();
        let mut nodes = vec![];

        const PREORDER_TRAVERSAL: fn(
            Option<Rc<RefCell<TreeNode>>>,
            &mut Vec<Rc<RefCell<TreeNode>>>,
        ) = |root: Option<Rc<RefCell<TreeNode>>>, nodes: &mut Vec<Rc<RefCell<TreeNode>>>| match root
        {
            None => {}
            Some(curr) => {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                nodes.push(curr);
                PREORDER_TRAVERSAL(left, nodes);
                PREORDER_TRAVERSAL(right, nodes);
            }
        };
        PREORDER_TRAVERSAL(root_clone.clone(), &mut nodes);

        for node in nodes.into_iter().skip(1) {
            if let Some(curr) = root_clone {
                curr.borrow_mut().right = Some(node);
                root_clone = curr.borrow().right.clone();
            }
        }
    }

    pub fn preorder_traversal_and_build(_root: &mut Option<Rc<RefCell<TreeNode>>>) {}
}
//leetcode submit region end(Prohibit modification and deletion)
