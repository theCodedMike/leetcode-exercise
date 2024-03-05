//Given the root of a Binary Search Tree (BST), return the minimum absolute
//difference between the values of any two different nodes in the tree.
//
//
// Example 1:
//
//
//Input: root = [4,2,6,1,3]
//Output: 1
//
//
// Example 2:
//
//
//Input: root = [1,0,48,null,null,12,49]
//Output: 1
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [2, 10⁴].
// 0 <= Node.val <= 10⁵
//
//
//
// Note: This question is the same as 783: https://leetcode.com/problems/
//minimum-distance-between-bst-nodes/
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Search
//Tree Binary Tree 👍 4222 👎 211

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::in_order_recur(root)
        //Self::in_order_iter(root)
    }

    fn in_order_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_abs_diff = i32::MAX;
        let mut prev_val = None;
        const IN_ORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut i32, &mut Option<i32>) =
            |root, min_abs_diff, prev_val| {
                if let Some(curr) = root {
                    IN_ORDER(curr.borrow_mut().left.take(), min_abs_diff, prev_val);

                    let curr_val = curr.borrow().val;
                    prev_val.map(|prev_val| {
                        let diff = curr_val - prev_val;
                        if diff < *min_abs_diff {
                            *min_abs_diff = diff;
                        }
                    });
                    *prev_val = Some(curr_val);

                    IN_ORDER(curr.borrow_mut().right.take(), min_abs_diff, prev_val);
                }
            };

        IN_ORDER(root, &mut min_abs_diff, &mut prev_val);

        min_abs_diff
    }

    fn in_order_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut min_abs_diff = i32::MAX;
        let mut prev_val = None;

        if let Some(root) = root {
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
                        prev_val.map(|prev_val| {
                            let diff = curr_val - prev_val;
                            if diff < min_abs_diff {
                                min_abs_diff = diff;
                            }
                        });
                        prev_val = Some(curr_val);
                    }
                }
            }
        }

        min_abs_diff
    }
}
//leetcode submit region end(Prohibit modification and deletion)
