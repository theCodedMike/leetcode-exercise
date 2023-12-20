//Given the root of a binary tree, check whether it is a mirror of itself (i.e.,
// symmetric around its center).
//
//
// Example 1:
//
//
//Input: root = [1,2,2,3,4,4,3]
//Output: true
//
//
// Example 2:
//
//
//Input: root = [1,2,2,null,3,null,3]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 1000].
// -100 <= Node.val <= 100
//
//
//
//Follow up: Could you solve it both recursively and iteratively?
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 14
//140 👎 322

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::bfs_recur(root)
        //Self::bfs_iter(root)
        //Self::dfs_iter(root)
    }

    fn bfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            const HELPER: fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool =
                |left, right| match (left, right) {
                    (Some(left), Some(right)) => {
                        left.borrow().val == right.borrow().val
                            && HELPER(left.borrow().left.clone(), right.borrow().right.clone())
                            && HELPER(left.borrow().right.clone(), right.borrow().left.clone())
                    }
                    (None, None) => true,
                    (_, _) => false,
                };

            HELPER(root.borrow().left.clone(), root.borrow().right.clone())
        } else {
            true
        }
    }

    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut queue = VecDeque::new();
            queue.push_back(root.borrow_mut().left.take());
            queue.push_back(root.borrow_mut().right.take());

            while !queue.is_empty() {
                match (queue.pop_front(), queue.pop_front()) {
                    (Some(left), Some(right)) => match (left, right) {
                        (Some(left), Some(right)) => {
                            if left.borrow().val != right.borrow().val {
                                return false;
                            }
                            queue.push_back(left.borrow_mut().left.take());
                            queue.push_back(right.borrow_mut().right.take());
                            queue.push_back(left.borrow_mut().right.take());
                            queue.push_back(right.borrow_mut().left.take());
                        }
                        (None, None) => {}
                        (_, _) => return false,
                    },
                    (_, _) => return false,
                }
            }
        }

        true
    }

    ///
    /// 一边是从根节点一直往左遍历，如果为空则向右，然后重复
    /// 一边是从根节点一直往右遍历，如果为空则向左，然后重复
    ///
    fn dfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            let mut l_r_root = root.borrow_mut().left.take();
            let mut r_l_root = root.borrow_mut().right.take();
            let mut l_r_stack = vec![];
            let mut r_l_stack = vec![];

            while l_r_root.is_some()
                || !l_r_stack.is_empty()
                || r_l_root.is_some()
                || !r_l_stack.is_empty()
            {
                match (l_r_root, r_l_root) {
                    (Some(l_r_node), Some(r_l_node)) => {
                        if l_r_node.borrow().val != r_l_node.borrow().val {
                            return false;
                        }
                        l_r_root = l_r_node.borrow_mut().left.take();
                        l_r_stack.push(l_r_node);
                        r_l_root = r_l_node.borrow_mut().right.take();
                        r_l_stack.push(r_l_node);
                    }
                    (None, None) => match (l_r_stack.pop(), r_l_stack.pop()) {
                        (Some(l_r_node), Some(r_l_node)) => {
                            l_r_root = l_r_node.borrow_mut().right.take();
                            r_l_root = r_l_node.borrow_mut().left.take();
                        }
                        (_, _) => return false,
                    },
                    (_, _) => return false,
                }
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
