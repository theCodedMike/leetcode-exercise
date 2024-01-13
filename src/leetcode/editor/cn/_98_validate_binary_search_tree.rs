//Given the root of a binary tree, determine if it is a valid binary search
//tree (BST).
//
// A valid BST is defined as follows:
//
//
// The left subtree of a node contains only nodes with keys less than the
//node's key.
// The right subtree of a node contains only nodes with keys greater than the
//node's key.
// Both the left and right subtrees must also be binary search trees.
//
//
//
// Example 1:
//
//
//Input: root = [2,1,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [5,1,4,null,null,3,6]
//Output: false
//Explanation: The root node's value is 5 but its right child's value is 4.
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
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 1545
//0 👎 1256

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //Self::dfs_recur_1(root)
        //Self::dfs_iter_1(root)
        Self::bfs_iter_1(root)
        //Self::dfs_recur_2(root)
        //Self::dfs_iter_2(root)
    }

    ///
    /// squeeze theorem(夹逼定理)
    ///
    fn dfs_recur_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        const DETERMINE: fn(Option<Rc<RefCell<TreeNode>>>, i64, i64) -> bool =
            |root, min, max| match root {
                None => true,
                Some(curr) => {
                    let curr_val = curr.borrow().val as i64;
                    if !(min < curr_val && curr_val < max) {
                        return false;
                    }

                    let left = curr.borrow_mut().left.take();
                    let right = curr.borrow_mut().right.take();

                    DETERMINE(left, min, curr_val) && DETERMINE(right, curr_val, max)
                }
            };

        DETERMINE(root, i64::MIN, i64::MAX)
    }

    fn dfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut stack = vec![(root, i64::MIN, i64::MAX)];

            while let Some((curr, min, max)) = stack.pop() {
                let curr_val = curr.borrow().val as i64;
                if !(min < curr_val && curr_val < max) {
                    return false;
                }

                if let Some(right) = curr.borrow_mut().right.take() {
                    stack.push((right, curr_val, max));
                }
                if let Some(left) = curr.borrow_mut().left.take() {
                    stack.push((left, min, curr_val));
                }
            }
        }

        true
    }

    fn bfs_iter_1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, i64::MIN, i64::MAX)]);
            while let Some((curr, min, max)) = queue.pop_front() {
                let curr_val = curr.borrow().val as i64;
                if !(min < curr_val && curr_val < max) {
                    return false;
                }

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, min, curr_val));
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, curr_val, max));
                }
            }
        }

        true
    }

    ///
    /// In-order traversal is ordered
    ///
    fn dfs_recur_2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut vals = vec![];
        const DETERMINE: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) -> bool =
            |root, vals| match root {
                None => true,
                Some(curr) => {
                    if !DETERMINE(curr.borrow_mut().left.take(), vals) {
                        return false;
                    }

                    let curr_val = curr.borrow().val;
                    if !vals.last().map_or(true, |&prev_val| curr_val > prev_val) {
                        return false;
                    }
                    vals.push(curr_val);

                    DETERMINE(curr.borrow_mut().right.take(), vals)
                }
            };

        DETERMINE(root, &mut vals)
    }

    fn dfs_iter_2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut vals = vec![];
            let mut stack = vec![Ok(root)];

            while let Some(curr) = stack.pop() {
                match curr {
                    Ok(node) => {
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push(Ok(right));
                        }

                        stack.push(Err(node.borrow().val));

                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push(Ok(left));
                        }
                    }
                    Err(curr_val) => {
                        if !vals.last().map_or(true, |&prev_val| curr_val > prev_val) {
                            return false;
                        }

                        vals.push(curr_val);
                    }
                }
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
