//Given the root of a binary tree, invert the tree, and return its root.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3,6,9]
//Output: [4,7,2,9,6,3,1]
//
//
// Example 2:
//
//
//Input: root = [2,1,3]
//Output: [2,3,1]
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
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 13
//481 👎 197

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::dfs_recur(root)
        //Self::dfs_iter(root)
        Self::bfs_iter(root)
    }

    fn dfs_recur(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        const RECUR: fn(Option<Rc<RefCell<TreeNode>>>) = |root| {
            if let Some(curr) = root {
                let mut ref_mut = curr.borrow_mut();
                let left = ref_mut.left.take();
                let right = ref_mut.right.take();
                ref_mut.left = right;
                ref_mut.right = left;

                RECUR(ref_mut.left.clone());
                RECUR(ref_mut.right.clone());
            }
        };

        RECUR(root.clone());

        root
    }

    fn dfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let mut stack = vec![root];

            while let Some(curr) = stack.pop() {
                let mut ref_mut = curr.borrow_mut();
                let left = ref_mut.left.take();
                let right = ref_mut.right.take();
                ref_mut.left = right;
                ref_mut.right = left;

                if let Some(right) = ref_mut.right.clone() {
                    stack.push(right);
                }
                if let Some(left) = ref_mut.left.clone() {
                    stack.push(left);
                }
            }
        }

        root
    }

    fn bfs_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root.clone() {
            let mut queue = VecDeque::from([root]);

            while let Some(curr) = queue.pop_front() {
                let mut ref_mut = curr.borrow_mut();
                let left = ref_mut.left.take();
                let right = ref_mut.right.take();
                ref_mut.left = right;
                ref_mut.right = left;

                if let Some(left) = ref_mut.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = ref_mut.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        root
    }
}
//leetcode submit region end(Prohibit modification and deletion)
