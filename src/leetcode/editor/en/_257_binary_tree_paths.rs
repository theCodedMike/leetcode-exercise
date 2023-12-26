//Given the root of a binary tree, return all root-to-leaf paths in any order.
//
// A leaf is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [1,2,3,null,5]
//Output: ["1->2->5","1->3"]
//
//
// Example 2:
//
//
//Input: root = [1]
//Output: ["1"]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 100].
// -100 <= Node.val <= 100
//
//
// Related Topics String Backtracking Tree Depth-First Search Binary Tree 👍 632
//7 👎 273

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;
//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        //Self::dfs_recur(root)
        Self::dfs_iter(root)
        //Self::bfs_iter(root)
    }

    fn convert(paths: Vec<Vec<i32>>) -> Vec<String> {
        paths
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|val| val.to_string())
                    .collect::<Vec<_>>()
                    .join("->")
            })
            .collect()
    }

    ///
    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n^2)
    ///
    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = vec![];
        const RECUR_HELPER: fn(Option<Rc<RefCell<TreeNode>>>, Vec<i32>, &mut Vec<Vec<i32>>) =
            |root, mut vals_vec, paths| {
                if let Some(curr) = root {
                    vals_vec.push(curr.borrow().val);

                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();
                    match (left, right) {
                        (None, None) => {
                            paths.push(vals_vec);
                        }
                        (left, right) => {
                            if left.is_some() {
                                RECUR_HELPER(left, vals_vec.clone(), paths);
                            }
                            if right.is_some() {
                                RECUR_HELPER(right, vals_vec, paths);
                            }
                        }
                    }
                }
            };

        RECUR_HELPER(root, vec![], &mut paths);

        Self::convert(paths)
    }

    ///
    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n^2)
    ///
    fn dfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut stack = vec![(root, vec![])];

            while let Some((curr, mut vals_vec)) = stack.pop() {
                vals_vec.push(curr.borrow().val);

                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        paths.push(vals_vec);
                    }
                    (left, right) => {
                        if let Some(right) = right {
                            stack.push((right, vals_vec.clone()));
                        }
                        if let Some(left) = left {
                            stack.push((left, vals_vec));
                        }
                    }
                }
            }
        }

        Self::convert(paths)
    }

    ///
    /// Time Complexity: O(n^2)
    /// Space Complexity: O(n^2)
    ///
    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, vec![])]);

            while let Some((curr, mut vals_vec)) = queue.pop_front() {
                vals_vec.push(curr.borrow().val);

                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        paths.push(vals_vec);
                    }
                    (left, right) => {
                        if let Some(left) = left {
                            queue.push_back((left, vals_vec.clone()));
                        }
                        if let Some(right) = right {
                            queue.push_back((right, vals_vec));
                        }
                    }
                }
            }
        }

        Self::convert(paths)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
