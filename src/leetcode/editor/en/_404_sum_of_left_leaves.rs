//Given the root of a binary tree, return the sum of all left leaves.
//
// A leaf is a node with no children. A left leaf is a leaf that is the left
//child of another node.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: 24
//Explanation: There are two left leaves in the binary tree, with values 9 and 1
//5 respectively.
//
//
// Example 2:
//
//
//Input: root = [1]
//Output: 0
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// -1000 <= Node.val <= 1000
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 49
//11 👎 283

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dfs_recur_1(root)
        //Self::dfs_recur_2(root)
        //Self::dfs_iter(root)
        Self::bfs_iter(root)
    }

    fn dfs_recur_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, bool, &mut i32) = |root, is_left, sum| {
            if let Some(curr) = root {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        if is_left {
                            *sum += curr.borrow().val;
                        }
                    }
                    (left, right) => {
                        if left.is_some() {
                            RECUR(left, true, sum);
                        }
                        if right.is_some() {
                            RECUR(right, false, sum);
                        }
                    }
                }
            }
        };

        RECUR(root, false, &mut sum);

        sum
    }

    fn dfs_recur_2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>, bool) -> i32 = |root, is_left| match root {
            None => 0,
            Some(curr) => {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        if is_left {
                            curr.borrow().val
                        } else {
                            0
                        }
                    }
                    (left, right) => RECUR(left, true) + RECUR(right, false),
                }
            }
        };

        RECUR(root, false)
    }

    fn dfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        if let Some(root) = root {
            let mut stack = vec![(root, false)];

            while let Some((curr, is_left)) = stack.pop() {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && is_left {
                    sum += curr.borrow().val;
                }
                if let Some(right) = right {
                    stack.push((right, false));
                }
                if let Some(left) = left {
                    stack.push((left, true));
                }
            }
        }

        sum
    }

    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, false)]);

            while let Some((curr, is_left)) = queue.pop_front() {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                if left.is_none() && right.is_none() && is_left {
                    sum += curr.borrow().val;
                }
                if let Some(left) = left {
                    queue.push_back((left, true));
                }
                if let Some(right) = right {
                    queue.push_back((right, false));
                }
            }
        }

        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
