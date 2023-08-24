//Given the root of a binary tree and an integer targetSum, return all root-to-
//leaf paths where the sum of the node values in the path equals targetSum. Each
//path should be returned as a list of the node values, not node references.
//
// A root-to-leaf path is a path starting from the root and ending at any leaf
//node. A leaf is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
//Output: [[5,4,11,2],[5,8,4,5]]
//Explanation: There are two paths whose sum equals targetSum:
//5 + 4 + 11 + 2 = 22
//5 + 8 + 4 + 5 = 22
//
//
// Example 2:
//
//
//Input: root = [1,2,3], targetSum = 5
//Output: []
//
//
// Example 3:
//
//
//Input: root = [1,2], targetSum = 0
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 5000].
// -1000 <= Node.val <= 1000
// -1000 <= targetSum <= 1000
//
//
// Related Topics Backtracking Tree Depth-First Search Binary Tree 👍 7405 👎 14
//2

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
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        Self::recursion_helper(root, target_sum).unwrap_or_default()
        //Self::bfs_helper(root, target_sum)
    }

    pub fn recursion_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
    ) -> Option<Vec<Vec<i32>>> {
        match root {
            None => None,
            Some(curr) => {
                let val = curr.borrow().val;
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        if val == target_sum {
                            Some(vec![vec![val]])
                        } else {
                            None
                        }
                    }
                    (l_child, r_child) => {
                        let left_res = Self::recursion_helper(l_child, target_sum - val);
                        let right_res = Self::recursion_helper(r_child, target_sum - val);
                        match (left_res, right_res) {
                            (None, None) => None,
                            (left_res, right_res) => {
                                let mut l_res = left_res.unwrap_or_default();
                                l_res.iter_mut().for_each(|v| v.insert(0, val));

                                let mut r_res = right_res.unwrap_or_default();
                                r_res.iter_mut().for_each(|v| v.insert(0, val));

                                l_res.append(&mut r_res);
                                Some(l_res)
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn bfs_helper(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        match root {
            None => {}
            Some(curr) => {
                let val = curr.borrow().val;
                let mut queue = VecDeque::from([(vec![val], val, curr)]);

                while !queue.is_empty() {
                    if let Some((mut nodes, sum, curr)) = queue.pop_front() {
                        let left = curr.borrow_mut().left.take();
                        let right = curr.borrow_mut().right.take();

                        match (left, right) {
                            (None, None) => {
                                if sum == target_sum {
                                    res.push(nodes);
                                }
                            }
                            (l_child, r_child) => {
                                if let Some(l_node) = l_child {
                                    let l_val = l_node.borrow().val;
                                    let l_sum = sum + l_val;
                                    let mut l_nodes = nodes.clone();
                                    l_nodes.push(l_val);
                                    queue.push_back((l_nodes, l_sum, l_node));
                                }

                                if let Some(r_node) = r_child {
                                    let r_val = r_node.borrow().val;
                                    let r_sum = sum + r_val;
                                    nodes.push(r_val);
                                    queue.push_back((nodes, r_sum, r_node));
                                }
                            }
                        }
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
