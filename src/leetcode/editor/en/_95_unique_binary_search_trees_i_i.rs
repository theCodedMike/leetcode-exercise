//Given an integer n, return all the structurally unique BST's (binary search
//trees), which has exactly n nodes of unique values from 1 to n. Return the answer
//in any order.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]
//]
//
//
// Example 2:
//
//
//Input: n = 1
//Output: [[1]]
//
//
//
// Constraints:
//
//
// 1 <= n <= 8
//
//
// Related Topics Dynamic Programming Backtracking Tree Binary Search Tree
//Binary Tree 👍 7141 👎 466

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
use std::rc::Rc;
impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // todo!
        vec![None; n as usize]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
