//Given the root of a binary tree, return the average value of the nodes on
//each level in the form of an array. Answers within 10⁻⁵ of the actual answer will
//be accepted.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: [3.00000,14.50000,11.00000]
//Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5,
//and on level 2 is 11.
//Hence return [3, 14.5, 11].
//
//
// Example 2:
//
//
//Input: root = [3,9,20,15,7]
//Output: [3.00000,14.50000,11.00000]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// -2³¹ <= Node.val <= 2³¹ - 1
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 50
//77 👎 312

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        //Self::dfs_recursion(root)
        //Self::dfs_iteration(root)

        //Self::bfs_iteration_1(root)
        Self::bfs_iteration_2(root)
    }

    fn dfs_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut level_sum = vec![];
        const PRE_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, usize, &mut Vec<(usize, i64)>) =
            |root, level, level_sum| {
                if let Some(curr) = root {
                    if level == level_sum.len() {
                        level_sum.push((0, 0));
                    }
                    level_sum[level].0 += 1;
                    level_sum[level].1 += curr.borrow().val as i64;

                    PRE_ORDER(curr.borrow_mut().left.take(), level + 1, level_sum);
                    PRE_ORDER(curr.borrow_mut().right.take(), level + 1, level_sum);
                }
            };

        PRE_ORDER(root, 0, &mut level_sum);

        level_sum
            .into_iter()
            .map(|(count, sum)| sum as f64 / count as f64)
            .collect()
    }

    fn dfs_iteration(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut level_sum = vec![];

        if let Some(root) = root {
            let mut stack = vec![Ok((root, 0))];

            while let Some(curr) = stack.pop() {
                match curr {
                    Ok((node, level)) => {
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push(Ok((right, level + 1)));
                        }
                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push(Ok((left, level + 1)));
                        }
                        stack.push(Err((node.borrow().val, level)));
                    }
                    Err((val, level)) => {
                        if level == level_sum.len() {
                            level_sum.push((0_usize, 0_i64));
                        }
                        level_sum[level].0 += 1;
                        level_sum[level].1 += val as i64;
                    }
                }
            }
        }

        level_sum
            .into_iter()
            .map(|(count, sum)| sum as f64 / count as f64)
            .collect()
    }

    fn bfs_iteration_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);
            let mut level_sum = (0_usize, 0_i64);
            let mut curr_level = 0;

            while let Some((curr, level)) = queue.pop_front() {
                if level != curr_level {
                    res.push(level_sum.1 as f64 / level_sum.0 as f64);
                    level_sum.0 = 0;
                    level_sum.1 = 0;
                }
                level_sum.0 += 1;
                level_sum.1 += curr.borrow().val as i64;
                curr_level = level;

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, level + 1));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, level + 1));
                }
            }
            res.push(level_sum.1 as f64 / level_sum.0 as f64);
        }

        res
    }

    fn bfs_iteration_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();

                let mut level_sum = 0_i64;
                for _ in 1..=level_len {
                    if let Some(curr) = queue.pop_front() {
                        level_sum += curr.borrow().val as i64;
                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }

                res.push(level_sum as f64 / level_len as f64);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
