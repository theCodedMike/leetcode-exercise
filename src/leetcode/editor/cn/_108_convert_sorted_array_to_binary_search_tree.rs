//Given an integer array nums where the elements are sorted in ascending order,
//convert it to a height-balanced binary search tree.
//
//
// Example 1:
//
//
//Input: nums = [-10,-3,0,5,9]
//Output: [0,-3,9,-10,null,5]
//Explanation: [0,-10,5,null,-3,null,9] is also accepted:
//
//
//
// Example 2:
//
//
//Input: nums = [1,3]
//Output: [3,1]
//Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums is sorted in a strictly increasing order.
//
//
// Related Topics Array Divide and Conquer Tree Binary Search Tree Binary Tree ?
//? 10084 👎 499

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
use std::ops::Index;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recursion_helper(&nums)
        //Self::iteration_helper_queue(nums)
        Self::iteration_helper_stack(nums)
    }

    fn recursion_helper(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }

        let root_idx = len / 2;
        let root = Rc::new(RefCell::new(TreeNode::new(nums[root_idx])));
        if len == 1 {
            return Some(root);
        }
        root.borrow_mut().left = Self::recursion_helper(nums.index(..root_idx));
        root.borrow_mut().right = Self::recursion_helper(nums.index(root_idx + 1..));

        Some(root)
    }

    fn iteration_helper_queue(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        if len == 1 {
            return Some(root);
        }

        let mut node_q = VecDeque::from([root.clone()]);
        let mut start_q = VecDeque::from([0]);
        let mut end_q = VecDeque::from([len]);
        while let Some(curr) = node_q.pop_front() {
            let start = start_q.pop_front().unwrap();
            let end = end_q.pop_front().unwrap();
            let mid = start + (end - start) / 2;
            curr.borrow_mut().val = nums[mid];

            let l_start = start;
            let l_end = mid;
            if l_start < l_end {
                let left = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().left = Some(left.clone());
                node_q.push_back(left);
                start_q.push_back(l_start);
                end_q.push_back(l_end);
            }

            let r_start = mid + 1;
            let r_end = end;
            if r_start < r_end {
                let right = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().right = Some(right.clone());
                node_q.push_back(right);
                start_q.push_back(r_start);
                end_q.push_back(r_end);
            }
        }

        Some(root)
    }

    fn iteration_helper_stack(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        if len == 1 {
            return Some(root);
        }

        let mut stack = vec![(root.clone(), 0, len)];
        while let Some((curr, start, end)) = stack.pop() {
            let mid = start + (end - start) / 2;
            curr.borrow_mut().val = nums[mid];

            if start < mid {
                let left = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().left = Some(left.clone());
                stack.push((left, start, mid))
            }

            if mid + 1 < end {
                let right = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().right = Some(right.clone());
                stack.push((right, mid + 1, end));
            }
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
