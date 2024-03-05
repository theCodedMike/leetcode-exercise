//You are given an integer array nums with no duplicates. A maximum binary tree
//can be built recursively from nums using the following algorithm:
//
//
// Create a root node whose value is the maximum value in nums.
// Recursively build the left subtree on the subarray prefix to the left of the
//maximum value.
// Recursively build the right subtree on the subarray suffix to the right of
//the maximum value.
//
//
// Return the maximum binary tree built from nums.
//
//
// Example 1:
//
//
//Input: nums = [3,2,1,6,0,5]
//Output: [6,3,5,null,2,0,null,null,1]
//Explanation: The recursive calls are as follow:
//- The largest value in [3,2,1,6,0,5] is 6. Left prefix is [3,2,1] and right
//suffix is [0,5].
//    - The largest value in [3,2,1] is 3. Left prefix is [] and right suffix
//is [2,1].
//        - Empty array, so no child.
//        - The largest value in [2,1] is 2. Left prefix is [] and right suffix
//is [1].
//            - Empty array, so no child.
//            - Only one element, so child is a node with value 1.
//    - The largest value in [0,5] is 5. Left prefix is [0] and right suffix is
//[].
//        - Only one element, so child is a node with value 0.
//        - Empty array, so no child.
//
//
// Example 2:
//
//
//Input: nums = [3,2,1]
//Output: [3,null,2,null,1]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 1000
// 0 <= nums[i] <= 1000
// All integers in nums are unique.
//
//
// Related Topics Array Divide and Conquer Stack Tree Monotonic Stack Binary
//Tree 👍 5079 👎 331

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_1(nums)
        //Self::recur_2(nums)
        Self::monotonic_stack_1(nums)
        //Self::monotonic_stack_2(nums)
    }

    ///
    /// Time complexity: O(n^2)
    /// Space complexity: O(n)
    ///
    fn recur_1(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const RECUR: fn(&[i32]) -> Option<Rc<RefCell<TreeNode>>> = |nums| {
            let len = nums.len();
            if len == 0 {
                return None;
            }

            let (max_idx, max_val) = nums
                .iter()
                .enumerate()
                .max_by(|&(_, a), &(_, b)| a.cmp(b))
                .map(|(idx, &val)| (idx, val))
                .unwrap_or_default();
            let root = Rc::new(RefCell::new(TreeNode::new(max_val)));
            if len == 1 {
                return Some(root);
            }

            let (left_nums, right_nums) = (&nums[..max_idx], &nums[max_idx + 1..]);
            root.borrow_mut().left = RECUR(left_nums);
            root.borrow_mut().right = RECUR(right_nums);

            Some(root)
        };

        RECUR(&nums)
    }

    ///
    /// Time complexity: O(n^2)
    /// Space complexity: O(n)
    ///
    fn recur_2(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        const RECUR: fn(&[i32], usize, usize) -> Option<Rc<RefCell<TreeNode>>> =
            |nums, l_idx, r_idx| {
                let len = r_idx - l_idx;
                if len == 0 {
                    return None;
                }

                let (max_idx, max_val) = nums[l_idx..r_idx]
                    .iter()
                    .enumerate()
                    .max_by(|&(_, a), &(_, b)| a.cmp(b))
                    .map(|(idx, val)| (idx + l_idx, *val))
                    .unwrap_or_default();
                let root = Rc::new(RefCell::new(TreeNode::new(max_val)));
                if len == 1 {
                    return Some(root);
                }

                root.borrow_mut().left = RECUR(nums, l_idx, max_idx);
                root.borrow_mut().right = RECUR(nums, max_idx + 1, r_idx);

                Some(root)
            };

        RECUR(&nums, 0, nums.len())
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    /// 需要深入了解
    ///
    fn monotonic_stack_1(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }

        let mut stack = Vec::with_capacity(len);
        let mut left = vec![usize::MAX; len];
        let mut right = vec![usize::MAX; len];
        let mut tree = Vec::with_capacity(len);

        for i in 0..len {
            tree.push(Rc::new(RefCell::new(TreeNode::new(nums[i]))));

            while let Some(&last) = stack.last() {
                if !(nums[i] > nums[last]) {
                    break;
                }
                right[last] = i;
                stack.pop();
            }
            if let Some(&last) = stack.last() {
                left[i] = last;
            }

            stack.push(i);
        }

        let mut root = None;
        for i in 0..len {
            if left[i] == usize::MAX && right[i] == usize::MAX {
                root = Some(tree[i].clone());
            } else if right[i] == usize::MAX
                || (left[i] != usize::MAX && nums[left[i]] < nums[right[i]])
            {
                tree[left[i]].borrow_mut().right = Some(tree[i].clone());
            } else {
                tree[right[i]].borrow_mut().left = Some(tree[i].clone());
            }
        }

        root
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    /// 需要深入了解
    ///
    fn monotonic_stack_2(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len == 0 {
            return None;
        }

        let mut stack: Vec<usize> = Vec::with_capacity(len);
        let mut tree = Vec::with_capacity(len);

        for i in 0..len {
            tree.push(Rc::new(RefCell::new(TreeNode::new(nums[i]))));

            while let Some(&last) = stack.last() {
                if !(nums[i] > nums[last]) {
                    break;
                }
                tree[i].borrow_mut().left = Some(tree[last].clone());
                stack.pop();
            }
            if let Some(&last) = stack.last() {
                tree[last].borrow_mut().right = Some(tree[i].clone())
            }

            stack.push(i);
        }

        Some(tree[stack[0]].clone())
    }
}
//leetcode submit region end(Prohibit modification and deletion)
