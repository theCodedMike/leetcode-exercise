//Given the root of a binary search tree and the lowest and highest boundaries
//as low and high, trim the tree so that all its elements lies in [low, high].
//Trimming the tree should not change the relative structure of the elements that
//will remain in the tree (i.e., any node's descendant should remain a descendant).
//It can be proven that there is a unique answer.
//
// Return the root of the trimmed binary search tree. Note that the root may
//change depending on the given bounds.
//
//
// Example 1:
//
//
//Input: root = [1,0,2], low = 1, high = 2
//Output: [1,null,2]
//
//
// Example 2:
//
//
//Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
//Output: [3,2,null,1]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// 0 <= Node.val <= 10⁴
// The value of each node in the tree is unique.
// root is guaranteed to be a valid binary search tree.
// 0 <= low <= high <= 10⁴
//
//
// Related Topics 树 深度优先搜索 二叉搜索树 二叉树 👍 903 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_helper(root, low, high)

        Self::iter_helper(root, low, high)
    }

    fn recur_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const TRIM: fn(Option<Rc<RefCell<TreeNode>>>, i32, i32) -> Option<Rc<RefCell<TreeNode>>> =
            |root, low, high| match root {
                None => None,
                Some(curr) => {
                    let curr_val = curr.borrow().val;
                    if curr_val < low {
                        TRIM(curr.borrow_mut().right.take(), low, high)
                    } else if curr_val > high {
                        TRIM(curr.borrow_mut().left.take(), low, high)
                    } else {
                        let left = curr.borrow_mut().left.take();
                        let right = curr.borrow_mut().right.take();
                        curr.borrow_mut().left = TRIM(left, low, high);
                        curr.borrow_mut().right = TRIM(right, low, high);

                        Some(curr)
                    }
                }
            };

        TRIM(root, low, high)
    }

    fn iter_helper(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        while let Some(curr) = root {
            let curr_val = curr.borrow().val;
            if curr_val < low {
                root = curr.borrow_mut().right.take();
            } else if curr_val > high {
                root = curr.borrow_mut().left.take();
            } else {
                root = Some(curr);
                break;
            }
        }

        if root.is_none() {
            return None;
        }

        let mut left_node = root.clone();
        while let Some(ref curr) = left_node {
            let left = curr.borrow().left.clone();
            if let Some(left) = left {
                if left.borrow().val < low {
                    curr.borrow_mut().left = left.borrow_mut().right.take();
                } else {
                    left_node = Some(left);
                }
            } else {
                break;
            };
        }

        let mut right_node = root.clone();
        while let Some(ref curr) = right_node {
            let right = curr.borrow().right.clone();
            if let Some(right) = right {
                if right.borrow().val > high {
                    curr.borrow_mut().right = right.borrow_mut().left.take();
                } else {
                    right_node = Some(right);
                }
            } else {
                break;
            };
        }

        root
    }
}
//leetcode submit region end(Prohibit modification and deletion)
