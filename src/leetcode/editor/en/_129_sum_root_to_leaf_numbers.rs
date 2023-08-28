//You are given the root of a binary tree containing digits from 0 to 9 only.
//
// Each root-to-leaf path in the tree represents a number.
//
//
// For example, the root-to-leaf path 1 -> 2 -> 3 represents the number 123.
//
//
// Return the total sum of all root-to-leaf numbers. Test cases are generated
//so that the answer will fit in a 32-bit integer.
//
// A leaf node is a node with no children.
//
//
// Example 1:
//
//
//Input: root = [1,2,3]
//Output: 25
//Explanation:
//The root-to-leaf path 1->2 represents the number 12.
//The root-to-leaf path 1->3 represents the number 13.
//Therefore, sum = 12 + 13 = 25.
//
//
// Example 2:
//
//
//Input: root = [4,9,0,5,1]
//Output: 1026
//Explanation:
//The root-to-leaf path 4->9->5 represents the number 495.
//The root-to-leaf path 4->9->1 represents the number 491.
//The root-to-leaf path 4->0 represents the number 40.
//Therefore, sum = 495 + 491 + 40 = 1026.
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// 0 <= Node.val <= 9
// The depth of the tree will not exceed 10.
//
//
// Related Topics Tree Depth-First Search Binary Tree 👍 7060 👎 109

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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut vals = vec![];
        Self::dfs_helper(root, &mut vals)
        //Self::bfs_helper(root)
    }

    pub fn dfs_helper(root: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) -> i32 {
        match root {
            None => -1,
            Some(curr) => {
                vals.push(curr.borrow().val);
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                let l_ret = Self::dfs_helper(left, vals);
                let r_ret = Self::dfs_helper(right, vals);
                match (l_ret, r_ret) {
                    (-1, -1) => {
                        let num = Self::convert_to_num(vals);
                        vals.pop();
                        num
                    }
                    (-1, num) | (num, -1) => {
                        vals.pop();
                        num
                    }
                    (l_num, r_num) => {
                        vals.pop();
                        l_num + r_num
                    }
                }
            }
        }
    }

    fn convert_to_num(vals: &mut Vec<i32>) -> i32 {
        let len = vals.len() - 1;
        vals.iter()
            .enumerate()
            .map(|(idx, &v)| 10_i32.pow((len - idx) as u32) * v)
            .sum()
    }

    pub fn bfs_helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        match root {
            None => {}
            Some(curr) => {
                let val = curr.borrow().val;
                let mut queue = VecDeque::from([(val, curr)]);
                while !queue.is_empty() {
                    if let Some((val, curr)) = queue.pop_front() {
                        let left = curr.borrow_mut().left.take();
                        let right = curr.borrow_mut().right.take();
                        match (left, right) {
                            (None, None) => {
                                sum += val;
                            }
                            (left, right) => {
                                if let Some(l) = left {
                                    let l_val = l.borrow().val;
                                    queue.push_back((val * 10 + l_val, l));
                                }

                                if let Some(r) = right {
                                    let r_val = r.borrow().val;
                                    queue.push_back((val * 10 + r_val, r));
                                }
                            }
                        }
                    }
                }
            }
        }

        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
