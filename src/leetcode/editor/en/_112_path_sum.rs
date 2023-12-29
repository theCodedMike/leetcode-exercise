//Given the root of a binary tree and an integer targetSum, return true if the
//tree has a root-to-leaf path such that adding up all the values along the path
//equals targetSum.
//
// A leaf is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
//Output: true
//Explanation: The root-to-leaf path with the target sum is shown.
//
//
// Example 2:
//
//
//Input: root = [1,2,3], targetSum = 5
//Output: false
//Explanation: There two root-to-leaf paths in the tree:
//(1 --> 2): The sum is 3.
//(1 --> 3): The sum is 4.
//There is no root-to-leaf path with sum = 5.
//
//
// Example 3:
//
//
//Input: root = [], targetSum = 0
//Output: false
//Explanation: Since the tree is empty, there are no root-to-leaf paths.
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
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 88
//76 👎 995

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        //Self::dfs_recur_1(root, target_sum)
        //Self::dfs_iter_1(root, target_sum)
        //Self::dfs_recur_2(root, target_sum)
        //Self::dfs_iter_2(root, target_sum)
        //Self::bfs_iter_1(root, target_sum)
        Self::bfs_iter_2(root, target_sum)
    }

    ///
    /// 1. Find all the paths
    /// 2. Compare the sum of one path with target_sum
    ///
    fn dfs_recur_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut paths = vec![];
        const RECUR: fn(root: Option<Rc<RefCell<TreeNode>>>, Vec<i32>, &mut Vec<Vec<i32>>) =
            |root, mut path, paths| {
                if let Some(curr) = root {
                    path.push(curr.borrow().val);
                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();

                    if left.is_none() && right.is_none() {
                        paths.push(path);
                    } else {
                        if left.is_some() {
                            RECUR(left, path.clone(), paths);
                        }
                        if right.is_some() {
                            RECUR(right, path, paths);
                        }
                    }
                }
            };

        RECUR(root, vec![], &mut paths);

        paths
            .into_iter()
            .any(|p| p.into_iter().sum::<i32>() == target_sum)
    }

    fn dfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut stack = vec![(root, vec![])];

            while let Some((curr, mut path)) = stack.pop() {
                path.push(curr.borrow().val);
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() {
                    paths.push(path);
                } else {
                    if let Some(right) = right {
                        stack.push((right, path.clone()));
                    }
                    if let Some(left) = left {
                        stack.push((left, path));
                    }
                }
            }
        }

        paths
            .into_iter()
            .any(|p| p.into_iter().sum::<i32>() == target_sum)
    }

    ///
    /// Accumulate val on each path and compare when reaching the leaf node
    ///
    fn dfs_recur_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, i32, i32) -> bool =
            |root, sum, target_sum| match root {
                None => false,
                Some(curr) => {
                    let curr_sum = curr.borrow().val + sum;
                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();

                    match (left, right) {
                        (None, None) => curr_sum == target_sum,
                        (None, right) => RECUR(right, curr_sum, target_sum),
                        (left, None) => RECUR(left, curr_sum, target_sum),
                        (left, right) => {
                            RECUR(left, curr_sum, target_sum) || RECUR(right, curr_sum, target_sum)
                        }
                    }
                }
            };

        RECUR(root, 0, target_sum)
    }

    fn dfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let mut stack = vec![(root, 0)];

            while let Some((curr, sum)) = stack.pop() {
                let curr_sum = curr.borrow().val + sum;
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && curr_sum == target_sum {
                    return true;
                }
                if let Some(right) = right {
                    stack.push((right, curr_sum));
                }
                if let Some(left) = left {
                    stack.push((left, curr_sum));
                }
            }
        }

        false
    }

    ///
    /// 1. Find all the paths
    /// 2. Compare the sum of one path with target_sum
    ///
    fn bfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut paths = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, vec![])]);

            while let Some((curr, mut path)) = queue.pop_front() {
                path.push(curr.borrow().val);
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                match (left, right) {
                    (None, None) => paths.push(path),
                    (left, right) => {
                        if let Some(left) = left {
                            queue.push_back((left, path.clone()));
                        }
                        if let Some(right) = right {
                            queue.push_back((right, path));
                        }
                    }
                }
            }
        }

        paths
            .into_iter()
            .any(|p| p.into_iter().sum::<i32>() == target_sum)
    }

    ///
    /// Accumulate val on each path and compare when reaching the leaf node
    ///
    fn bfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);

            while let Some((curr, sum)) = queue.pop_front() {
                let curr_sum = curr.borrow().val + sum;
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                match (left, right) {
                    (None, None) => {
                        if curr_sum == target_sum {
                            return true;
                        }
                    }
                    (left, right) => {
                        if let Some(left) = left {
                            queue.push_back((left, curr_sum));
                        }
                        if let Some(right) = right {
                            queue.push_back((right, curr_sum));
                        }
                    }
                }
            }
        }

        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
