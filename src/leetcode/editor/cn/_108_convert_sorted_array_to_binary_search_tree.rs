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
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_helper_1(nums)
        //Self::recur_helper_2(nums)

        //Self::iter_helper_1(nums)
        Self::iter_helper_2(nums)
    }

    fn recur_helper_1(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const CONVERT: fn(&[i32]) -> Option<Rc<RefCell<TreeNode>>> = |nums| {
            let len = nums.len();
            if len == 0 {
                return None;
            }

            let mid = len / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            if len == 1 {
                return Some(root);
            }

            root.borrow_mut().left = CONVERT(&nums[..mid]);
            root.borrow_mut().right = CONVERT(&nums[mid + 1..]);

            Some(root)
        };

        CONVERT(&nums)
    }

    fn recur_helper_2(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const CONVERT: fn(&[i32], usize, usize) -> Option<Rc<RefCell<TreeNode>>> =
            |nums, left, right| {
                if left == right {
                    return None;
                }

                let mid = (left + right) / 2;
                let root = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
                if right - left == 1 {
                    return Some(root);
                }

                root.borrow_mut().left = CONVERT(&nums, left, mid);
                root.borrow_mut().right = CONVERT(&nums, mid + 1, right);

                Some(root)
            };

        CONVERT(&nums, 0, nums.len())
    }

    fn iter_helper_1(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        let mut queue = VecDeque::from([(root.clone(), 0, nums.len())]);

        while let Some((curr, l_idx, r_idx)) = queue.pop_front() {
            let mid = (l_idx + r_idx) / 2;
            curr.borrow_mut().val = nums[mid];

            if l_idx < mid {
                let l_node = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().left = Some(l_node.clone());
                queue.push_back((l_node, l_idx, mid));
            }

            if mid + 1 < r_idx {
                let r_node = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().right = Some(r_node.clone());
                queue.push_back((r_node, mid + 1, r_idx));
            }
        }

        Some(root)
    }

    fn iter_helper_2(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(nums[0])));
        let mut stack = vec![(root.clone(), 0, nums.len())];

        while let Some((curr, l_idx, r_idx)) = stack.pop() {
            let mid = (l_idx + r_idx) / 2;
            curr.borrow_mut().val = nums[mid];

            if l_idx < mid {
                let l_node = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().left = Some(l_node.clone());
                stack.push((l_node, l_idx, mid));
            }

            if mid + 1 < r_idx {
                let r_node = Rc::new(RefCell::new(TreeNode::new(nums[0])));
                curr.borrow_mut().right = Some(r_node.clone());
                stack.push((r_node, mid + 1, r_idx));
            }
        }

        Some(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
