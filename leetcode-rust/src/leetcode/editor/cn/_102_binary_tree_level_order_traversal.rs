//Given the root of a binary tree, return the level order traversal of its
//nodes' values. (i.e., from left to right, level by level).
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: [[3],[9,20],[15,7]]
//
//
// Example 2:
//
//
//Input: root = [1]
//Output: [[1]]
//
//
// Example 3:
//
//
//Input: root = []
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 2000].
// -1000 <= Node.val <= 1000
//
//
// Related Topics Tree Breadth-First Search Binary Tree 👍 14037 👎 277

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        //Self::recursion_impl(root)
        //Self::iteration_impl_1(root)
        Self::iteration_impl_2(root)
    }

    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        const RECURSION_IMPL: fn(Option<Rc<RefCell<TreeNode>>>, usize, &mut Vec<Vec<i32>>) =
            |root, level, res| match root {
                None => {}
                Some(curr) => {
                    if level == res.len() {
                        res.push(vec![]);
                    }
                    res[level].push(curr.borrow().val);

                    if let Some(left) = curr.borrow_mut().left.take() {
                        RECURSION_IMPL(Some(left), level + 1, res);
                    }
                    if let Some(right) = curr.borrow_mut().right.take() {
                        RECURSION_IMPL(Some(right), level + 1, res);
                    }
                }
            };

        RECURSION_IMPL(root, 0, &mut res);

        res
    }

    fn iteration_impl_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(0, root)]);

            while let Some((level, curr)) = queue.pop_front() {
                if level == res.len() {
                    res.push(vec![]);
                }
                res[level].push(curr.borrow().val);

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((level + 1, left));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((level + 1, right));
                }
            }
        }

        res
    }

    fn iteration_impl_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();

                let mut level_vec = vec![];
                for _ in 0..level_len {
                    if let Some(curr) = queue.pop_front() {
                        level_vec.push(curr.borrow().val);

                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }
                res.push(level_vec);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
