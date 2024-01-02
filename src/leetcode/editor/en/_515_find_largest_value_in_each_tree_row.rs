//Given the root of a binary tree, return an array of the largest value in each
//row of the tree (0-indexed).
//
//
// Example 1:
//
//
//Input: root = [1,3,2,5,3,null,9]
//Output: [1,3,9]
//
//
// Example 2:
//
//
//Input: root = [1,2,3]
//Output: [1,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree will be in the range [0, 10⁴].
// -2³¹ <= Node.val <= 2³¹ - 1
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 35
//03 👎 110

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Self::dfs_recur_pre_order(root)
        //Self::dfs_iter_pre_order_1(root)
        //Self::dfs_iter_pre_order_2(root)
        //Self::dfs_iter_pre_order_3(root)

        //Self::bfs_iter_1(root)
        Self::bfs_iter_2(root)
    }

    ///
    /// DFS - Recursion(Pre-Order)
    ///
    fn dfs_recur_pre_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        const PRE_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, usize, &mut Vec<i32>) =
            |root, level, res| {
                if let Some(curr) = root {
                    if level == res.len() {
                        res.push(i32::MIN);
                    }
                    let curr_val = curr.borrow().val;
                    if res[level] < curr_val {
                        res[level] = curr_val;
                    }
                    PRE_ORDER(curr.borrow_mut().left.take(), level + 1, res);
                    PRE_ORDER(curr.borrow_mut().right.take(), level + 1, res);
                }
            };

        PRE_ORDER(root, 0, &mut res);

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut root = (root, 0);
        let mut stack = vec![];

        while root.0.is_some() || !stack.is_empty() {
            while let Some(curr) = root.0 {
                let level = root.1;
                if level == res.len() {
                    res.push(i32::MIN);
                }
                let curr_val = curr.borrow().val;
                if res[level] < curr_val {
                    res[level] = curr_val;
                }

                root = (curr.borrow_mut().left.take(), level + 1);
                stack.push((curr, level));
            }

            if let Some((curr, level)) = stack.pop() {
                root = (curr.borrow_mut().right.take(), level + 1);
            }
        }

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut root = (root, 0);
        let mut stack = vec![];

        while root.0.is_some() || !stack.is_empty() {
            if let Some(curr) = root.0 {
                let level = root.1;
                if level == res.len() {
                    res.push(i32::MIN);
                }
                let curr_val = curr.borrow().val;
                if res[level] < curr_val {
                    res[level] = curr_val;
                }

                root = (curr.borrow_mut().left.take(), level + 1);
                stack.push((curr, level));
            } else {
                if let Some((curr, level)) = stack.pop() {
                    root = (curr.borrow_mut().right.take(), level + 1);
                }
            }
        }

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![(Ok(root), 0)];

            while let Some((curr, level)) = stack.pop() {
                match curr {
                    Ok(node) => {
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push((Ok(right), level + 1));
                        }
                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push((Ok(left), level + 1));
                        }
                        stack.push((Err(node.borrow().val), level));
                    }
                    Err(curr_val) => {
                        if level == res.len() {
                            res.push(i32::MIN);
                        }
                        if res[level] < curr_val {
                            res[level] = curr_val;
                        }
                    }
                }
            }
        }

        res
    }

    ///
    /// BFS - Iteration(Level Order)
    ///
    fn bfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);

            while let Some((curr, level)) = queue.pop_front() {
                if level == res.len() {
                    res.push(i32::MIN);
                }
                let curr_val = curr.borrow().val;
                if res[level] < curr_val {
                    res[level] = curr_val;
                }

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, level + 1));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, level + 1));
                }
            }
        }

        res
    }

    ///
    /// BFS - Iteration(Level Order)
    ///
    fn bfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();

                let mut level_largest = i32::MIN;
                for _ in 0..level_len {
                    if let Some(curr) = queue.pop_front() {
                        let curr_val = curr.borrow().val;
                        if curr_val > level_largest {
                            level_largest = curr_val;
                        }

                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }

                res.push(level_largest);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
