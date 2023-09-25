//Given the root of a binary tree, imagine yourself standing on the right side
//of it, return the values of the nodes you can see ordered from top to bottom.
//
//
// Example 1:
//
//
//Input: root = [1,2,3,null,5,null,4]
//Output: [1,3,4]
//
//
// Example 2:
//
//
//Input: root = [1,null,3]
//Output: [1,3]
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
// The number of nodes in the tree is in the range [0, 100].
// -100 <= Node.val <= 100
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 11
//058 👎 724

#![allow(dead_code)]

///
/// 查看每层的最后一个节点的值
///
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
    pub fn new2(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}
//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::bfs_helper(root)
    }

    pub fn bfs_helper(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        match root {
            None => {}
            Some(root) => {
                let mut queue = VecDeque::from([(root, 1)]);
                let mut prev_level = 1;
                let mut prev_node: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(0)));

                while let Some((node, curr_level)) = queue.pop_front() {
                    if let Some(left) = node.borrow_mut().left.take() {
                        queue.push_back((left, curr_level + 1));
                    }
                    if let Some(right) = node.borrow_mut().right.take() {
                        queue.push_back((right, curr_level + 1));
                    }

                    if curr_level != prev_level {
                        prev_level = curr_level;
                        res.push(prev_node.borrow().val);
                    }
                    prev_node = node;
                    if queue.is_empty() {
                        res.push(prev_node.borrow().val);
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
