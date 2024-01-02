//Given two integer arrays preorder and inorder where preorder is the preorder
//traversal of a binary tree and inorder is the inorder traversal of the same tree,
// construct and return the binary tree.
//
//
// Example 1:
//
//
//Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
//Output: [3,9,20,null,null,15,7]
//
//
// Example 2:
//
//
//Input: preorder = [-1], inorder = [-1]
//Output: [-1]
//
//
//
// Constraints:
//
//
// 1 <= preorder.length <= 3000
// inorder.length == preorder.length
// -3000 <= preorder[i], inorder[i] <= 3000
// preorder and inorder consist of unique values.
// Each value of inorder also appears in preorder.
// preorder is guaranteed to be the preorder traversal of the tree.
// inorder is guaranteed to be the inorder traversal of the tree.
//
//
// Related Topics Array Hash Table Divide and Conquer Tree Binary Tree 👍 13663
//👎 409

#![allow(dead_code)]
#![allow(unused)]

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_1(preorder, inorder)
        //Self::recur_2(preorder, inorder)
        Self::iter_1(preorder, inorder)
    }

    fn recur_1(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const RECUR: fn(&[i32], &[i32]) -> Option<Rc<RefCell<TreeNode>>> = |preorder, inorder| {
            let len = preorder.len();
            if len == 0 {
                return None;
            }

            let root_val = preorder[0];
            let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
            if len == 1 {
                return Some(root);
            }

            // len > 1
            let mut childs_inorder = inorder.splitn(2, |&v| v == root_val);
            let left_inorder = childs_inorder.next().unwrap_or_default();
            let right_inorder = childs_inorder.next().unwrap_or_default();
            let (left_preorder, right_preorder) = preorder[1..].split_at(left_inorder.len());

            root.borrow_mut().left = RECUR(left_preorder, left_inorder);
            root.borrow_mut().right = RECUR(right_preorder, right_inorder);

            Some(root)
        };

        RECUR(&preorder, &inorder)
    }

    fn recur_2(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();
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
        ) -> Option<Rc<RefCell<TreeNode>>> = |preorder,
                                              preorder_l_idx,
                                              preorder_r_idx,
                                              inorder,
                                              inorder_l_idx,
                                              inorder_r_idx,
                                              map| {
            let mut len = preorder_r_idx - preorder_l_idx;
            if len == 0 {
                return None;
            }

            let root_val = preorder[preorder_l_idx];
            let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
            if len == 1 {
                return Some(root);
            }

            // len > 1
            let idx_at_inorder = map[&root_val];
            let left_inorder_len = idx_at_inorder - inorder_l_idx;
            let right_inorder_len = inorder_r_idx - idx_at_inorder - 1;

            root.borrow_mut().left = RECUR(
                preorder,
                preorder_l_idx + 1,
                preorder_l_idx + 1 + left_inorder_len,
                inorder,
                inorder_l_idx,
                idx_at_inorder,
                map,
            );
            root.borrow_mut().right = RECUR(
                preorder,
                preorder_r_idx - right_inorder_len,
                preorder_r_idx,
                inorder,
                idx_at_inorder + 1,
                inorder_r_idx,
                map,
            );

            Some(root)
        };

        RECUR(&preorder, 0, len, &inorder, 0, len, &map)
    }

    fn iter_1(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack = vec![root.clone()];
        let mut inorder_idx = 0;

        for preorder_val in preorder.into_iter().skip(1) {
            if let Some(last) = stack.last_mut() {
                if last.borrow().val != inorder[inorder_idx] {
                    let left = Rc::new(RefCell::new(TreeNode::new(preorder_val)));
                    last.borrow_mut().left = Some(left.clone());
                    stack.push(left);
                } else {
                    let mut last_curr = None;

                    while let Some(curr) = stack.pop() {
                        last_curr = Some(curr);
                        inorder_idx += 1;
                        if let Some(last) = stack.last() {
                            if last.borrow().val != inorder[inorder_idx] {
                                break;
                            }
                        }
                    }

                    last_curr.map(|curr| {
                        let right = Rc::new(RefCell::new(TreeNode::new(preorder_val)));
                        curr.borrow_mut().right = Some(right.clone());
                        stack.push(right);
                    });
                }
            }
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
