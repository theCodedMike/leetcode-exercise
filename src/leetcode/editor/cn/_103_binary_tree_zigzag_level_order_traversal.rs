//Given the root of a binary tree, return the zigzag level order traversal of
//its nodes' values. (i.e., from left to right, then right to left for the next
//level and alternate between).
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: [[3],[20,9],[15,7]]
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
// -100 <= Node.val <= 100
//
//
// Related Topics Tree Breadth-First Search Binary Tree 👍 9864 👎 254

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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        match root {
            None => return res,
            Some(curr) => {
                let mut queue = VecDeque::new();
                queue.push_back((curr, 0));

                while !queue.is_empty() {
                    if let Some((curr, level)) = queue.pop_front() {
                        let val = curr.borrow().val;
                        match res.get_mut(level) {
                            None => {
                                res.push(vec![val]);
                            }
                            Some(vec) => {
                                if level % 2 == 1 {
                                    vec.insert(0, val);
                                } else {
                                    vec.push(val);
                                }
                            }
                        }

                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back((left, level + 1));
                        }

                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back((right, level + 1));
                        }
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
