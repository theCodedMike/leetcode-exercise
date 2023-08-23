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
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::recursion_helper(root, target_sum)
        //Self::bfs_helper(root, target_sum)
    }

    fn recursion_helper(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(curr) => {
                let val = curr.borrow().val;
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                match (left, right) {
                    (None, None) => {
                        if val == target_sum {
                            true
                        } else {
                            false
                        }
                    }
                    (l_child, r_child) => {
                        let l_eq = Self::recursion_helper(l_child, target_sum - val);
                        let r_eq = Self::recursion_helper(r_child, target_sum - val);
                        l_eq || r_eq
                    }
                }
            }
        }
    }

    fn bfs_helper(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(curr) => {
                let val = curr.borrow().val;
                let mut queue = VecDeque::from([(val, curr)]);
                while !queue.is_empty() {
                    if let Some((sum, curr)) = queue.pop_front() {
                        let left_child = curr.borrow_mut().left.take();
                        let right_child = curr.borrow_mut().right.take();
                        match (left_child, right_child) {
                            (None, None) => {
                                if sum == target_sum {
                                    return true;
                                }
                            }
                            (l_child, r_child) => {
                                if let Some(left) = l_child {
                                    let left_val = left.borrow().val;
                                    queue.push_back((sum + left_val, left));
                                }
                                if let Some(right) = r_child {
                                    let right_val = right.borrow().val;
                                    queue.push_back((sum + right_val, right));
                                }
                            }
                        }
                    }
                }

                false
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
