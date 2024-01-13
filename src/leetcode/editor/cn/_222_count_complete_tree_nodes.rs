//Given the root of a complete binary tree, return the number of the nodes in
//the tree.
//
// According to Wikipedia, every level, except possibly the last, is completely
//filled in a complete binary tree, and all nodes in the last level are as far
//left as possible. It can have between 1 and 2ʰ nodes inclusive at the last level h.
//
//
// Design an algorithm that runs in less than O(n) time complexity.
//
//
// Example 1:
//
//
//Input: root = [1,2,3,4,5,6]
//Output: 6
//
//
// Example 2:
//
//
//Input: root = []
//Output: 0
//
//
// Example 3:
//
//
//Input: root = [1]
//Output: 1
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 5 * 10⁴].
// 0 <= Node.val <= 5 * 10⁴
// The tree is guaranteed to be complete.
//
//
// Related Topics Binary Search Bit Manipulation Tree Binary Tree 👍 8293 👎 469
//

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //Self::dfs_recur(root)
        //Self::bfs_iter(root)
        Self::binary_search(root)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(log(n))
    ///
    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const COUNT: fn(Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| {
            if let Some(root) = root {
                COUNT(root.borrow().left.clone()) + COUNT(root.borrow().right.clone()) + 1
            } else {
                0
            }
        };

        COUNT(root)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(n)
    ///
    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while let Some(curr) = queue.pop_front() {
                count += 1;

                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back(left);
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back(right)
                }
            }
        }

        count
    }

    ///
    /// Time Complexity: O(log(n) * log(n))
    /// Space Complexity: O(1)
    ///
    fn binary_search(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let calc_level: fn(root: Option<Rc<RefCell<TreeNode>>>) -> u32 = |mut root| {
            let mut level = 0_u32;

            while let Some(curr) = root {
                root = curr.borrow().left.clone();
                level += 1;
            }

            level
        };
        let level = calc_level(root.clone());
        let mut min_count = 2_i32.pow(level - 1);
        let mut max_count = 2_i32.pow(level);
        let exist: fn(Option<Rc<RefCell<TreeNode>>>, i32) -> bool = |mut root, expected_count| {
            for c in format!("{:b}", expected_count).chars().skip(1) {
                if let Some(curr) = root {
                    if c == '1' {
                        root = curr.borrow().right.clone();
                    } else {
                        root = curr.borrow().left.clone();
                    }
                    if root.is_none() {
                        return false;
                    }
                }
            }

            true
        };

        while min_count < max_count {
            let mid = (min_count + max_count) / 2;

            if exist(root.clone(), mid) {
                min_count = mid + 1;
            } else {
                max_count = mid;
            }
        }

        min_count - 1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
