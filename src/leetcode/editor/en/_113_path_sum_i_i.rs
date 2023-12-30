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
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        //Self::dfs_recur_1(root, target_sum)
        //Self::dfs_iter_1(root, target_sum)
        //Self::dfs_recur_2(root, target_sum)
        Self::bfs_iter(root, target_sum)
    }

    fn dfs_recur_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, i32, i32, Vec<i32>, &mut Vec<Vec<i32>>) =
            |root, target_sum, sum, mut path, paths| {
                if let Some(curr) = root {
                    let curr_val = curr.borrow().val;
                    let curr_sum = sum + curr_val;
                    path.push(curr_val);

                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();

                    if left.is_none() && right.is_none() && curr_sum == target_sum {
                        paths.push(path);
                    } else {
                        if left.is_some() {
                            RECUR(left, target_sum, curr_sum, path.clone(), paths);
                        }
                        if right.is_some() {
                            RECUR(right, target_sum, curr_sum, path, paths);
                        }
                    }
                }
            };

        RECUR(root, target_sum, 0, vec![], &mut paths);

        paths
    }

    fn dfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut stack = vec![(root, 0, vec![])];

            while let Some((curr, sum, mut path)) = stack.pop() {
                let curr_val = curr.borrow().val;
                let curr_sum = sum + curr_val;
                path.push(curr_val);

                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && curr_sum == target_sum {
                    paths.push(path);
                } else {
                    if let Some(right) = right {
                        stack.push((right, curr_sum, path.clone()));
                    }
                    if let Some(left) = left {
                        stack.push((left, curr_sum, path));
                    }
                }
            }
        }

        paths
    }

    fn dfs_recur_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, i32, i32) -> Vec<Vec<i32>> =
            |root, target_sum, sum| match root {
                None => vec![],
                Some(curr) => {
                    let curr_val = curr.borrow().val;
                    let curr_sum = curr_val + sum;
                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();

                    if left.is_none() && right.is_none() {
                        if curr_sum == target_sum {
                            vec![vec![curr_val]]
                        } else {
                            vec![]
                        }
                    } else {
                        let mut l_paths = RECUR(left, target_sum, curr_sum);
                        let mut r_paths = RECUR(right, target_sum, curr_sum);
                        l_paths.iter_mut().for_each(|p| p.insert(0, curr_val));
                        r_paths.iter_mut().for_each(|p| p.insert(0, curr_val));
                        l_paths.append(&mut r_paths);
                        l_paths
                    }
                }
            };

        RECUR(root, target_sum, 0)
    }

    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0, vec![])]);

            while let Some((curr, sum, mut path)) = queue.pop_front() {
                let curr_val = curr.borrow().val;
                let curr_sum = curr_val + sum;
                path.push(curr_val);

                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && curr_sum == target_sum {
                    paths.push(path);
                } else {
                    if let Some(left) = left {
                        queue.push_back((left, curr_sum, path.clone()));
                    }
                    if let Some(right) = right {
                        queue.push_back((right, curr_sum, path));
                    }
                }
            }
        }

        paths
    }
}
//leetcode submit region end(Prohibit modification and deletion)
