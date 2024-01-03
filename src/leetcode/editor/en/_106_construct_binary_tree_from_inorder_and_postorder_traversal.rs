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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_1(inorder, postorder)
        //Self::recur_2(inorder, postorder)
        Self::iter_1(inorder, postorder)
    }

    fn recur_1(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const RECUR: fn(&[i32], &[i32]) -> Option<Rc<RefCell<TreeNode>>> = |postorder, inorder| {
            let len = postorder.len();
            if len == 0 {
                return None;
            }

            let root_val = postorder[len - 1];
            let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
            if len == 1 {
                return Some(root);
            }

            let mut childs_inorder = inorder.splitn(2, |&v| v == root_val);
            let left_inorder = childs_inorder.next().unwrap_or_default();
            let right_inorder = childs_inorder.next().unwrap_or_default();
            let (left_postorder, right_postorder) =
                postorder[..len - 1].split_at(left_inorder.len());

            root.borrow_mut().left = RECUR(left_postorder, left_inorder);
            root.borrow_mut().right = RECUR(right_postorder, right_inorder);

            Some(root)
        };

        RECUR(&postorder, &inorder)
    }

    fn recur_2(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = postorder.len();
        let map =
            inorder
                .iter()
                .enumerate()
                .fold(HashMap::with_capacity(len), |mut map, (idx, &v)| {
                    map.insert(v, idx);
                    map
                });
        const RECUR: fn(
            &[i32],
            usize,
            usize,
            &[i32],
            usize,
            usize,
            &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> = |postorder,
                                              postorder_l_idx,
                                              postorder_r_idx,
                                              inorder,
                                              inorder_l_idx,
                                              inorder_r_idx,
                                              map| {
            let len = postorder_r_idx - postorder_l_idx;
            if len == 0 {
                return None;
            }

            let root_val = postorder[postorder_r_idx - 1];
            let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
            if len == 1 {
                return Some(root);
            }

            let idx_at_inorder = map[&root_val];
            let left_inorder_len = idx_at_inorder - inorder_l_idx;

            root.borrow_mut().left = RECUR(
                postorder,
                postorder_l_idx,
                postorder_l_idx + left_inorder_len,
                inorder,
                inorder_l_idx,
                idx_at_inorder,
                map,
            );
            root.borrow_mut().right = RECUR(
                postorder,
                postorder_l_idx + left_inorder_len,
                postorder_r_idx - 1,
                inorder,
                idx_at_inorder + 1,
                inorder_r_idx,
                map,
            );

            Some(root)
        };

        RECUR(&postorder, 0, len, &inorder, 0, len, &map)
    }

    fn iter_1(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = postorder.len();
        if len == 0 {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(postorder[len - 1])));
        let mut stack = vec![root.clone()];
        let mut inorder_idx = len - 1;

        for postorder_val in postorder.into_iter().rev().skip(1) {
            if let Some(last) = stack.last_mut() {
                if last.borrow().val != inorder[inorder_idx] {
                    let right = Rc::new(RefCell::new(TreeNode::new(postorder_val)));
                    last.borrow_mut().right = Some(right.clone());
                    stack.push(right);
                } else {
                    let mut last_curr = None;

                    while let Some(curr) = stack.pop() {
                        last_curr = Some(curr);
                        inorder_idx -= 1;
                        if let Some(last) = stack.last() {
                            if last.borrow().val != inorder[inorder_idx] {
                                break;
                            }
                        }
                    }

                    last_curr.map(|curr| {
                        let left = Rc::new(RefCell::new(TreeNode::new(postorder_val)));
                        curr.borrow_mut().left = Some(left.clone());
                        stack.push(left);
                    });
                }
            }
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
