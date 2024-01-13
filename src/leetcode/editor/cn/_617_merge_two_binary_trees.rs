//You are given two binary trees root1 and root2.
//
// Imagine that when you put one of them to cover the other, some nodes of the
//two trees are overlapped while the others are not. You need to merge the two
//trees into a new binary tree. The merge rule is that if two nodes overlap, then sum
//node values up as the new value of the merged node. Otherwise, the NOT null
//node will be used as the node of the new tree.
//
// Return the merged tree.
//
// Note: The merging process must start from the root nodes of both trees.
//
//
// Example 1:
//
//
//Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
//Output: [3,4,5,5,4,null,7]
//
//
// Example 2:
//
//
//Input: root1 = [1], root2 = [1,2]
//Output: [2,2]
//
//
//
// Constraints:
//
//
// The number of nodes in both trees is in the range [0, 2000].
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 86
//10 👎 288

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::dfs_recur_create_new(root1, root2)
        //Self::dfs_iter_create_new(root1, root2)
        //Self::dfs_recur_reuse(root1, root2)
        //Self::dfs_iter_reuse(root1, root2)
        //Self::bfs_iter_create_new(root1, root2)
        Self::bfs_iter_reuse(root1, root2)
    }

    ///
    /// DFS, recursion, create a new node
    ///
    fn dfs_recur_create_new(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const MERGE: fn(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> = |root1, root2| match (root1, root2) {
            (None, r2) => r2,
            (r1, None) => r1,
            (Some(r1), Some(r2)) => {
                let mut r1 = r1.borrow_mut();
                let mut r2 = r2.borrow_mut();
                let root = Rc::new(RefCell::new(TreeNode::new(r1.val + r2.val)));

                root.borrow_mut().left = MERGE(r1.left.take(), r2.left.take());
                root.borrow_mut().right = MERGE(r1.right.take(), r2.right.take());

                Some(root)
            }
        };

        MERGE(root1, root2)
    }

    ///
    /// DFS, iteration, create a new node
    ///
    fn dfs_iter_create_new(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        let mut stack = vec![(None, root1, root2, false)];

        while let Some((parent, r1, r2, is_left)) = stack.pop() {
            let node = match (r1, r2) {
                (None, r2) => r2,
                (r1, None) => r1,
                (Some(r1), Some(r2)) => {
                    let mut r1 = r1.borrow_mut();
                    let mut r2 = r2.borrow_mut();

                    let node = Some(Rc::new(RefCell::new(TreeNode::new(r1.val + r2.val))));
                    if r1.right.is_some() || r2.right.is_some() {
                        stack.push((node.clone(), r1.right.take(), r2.right.take(), false));
                    }
                    if r1.left.is_some() || r2.left.is_some() {
                        stack.push((node.clone(), r1.left.take(), r2.left.take(), true));
                    }

                    node
                }
            };

            if let Some(p) = parent {
                if is_left {
                    p.borrow_mut().left = node;
                } else {
                    p.borrow_mut().right = node;
                }
            } else {
                root = node;
            }
        }

        root
    }

    ///
    /// DFS, recursion, reuse root1
    ///
    fn dfs_recur_reuse(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const MERGE: fn(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> = |root1, root2| match (root1, root2) {
            (None, r2) => r2,
            (r1, None) => r1,
            (Some(r1), Some(r2)) => {
                r1.borrow_mut().val += r2.borrow().val;
                let r1_l = r1.borrow_mut().left.take();
                let r2_l = r2.borrow_mut().left.take();
                let r1_r = r1.borrow_mut().right.take();
                let r2_r = r2.borrow_mut().right.take();

                r1.borrow_mut().left = MERGE(r1_l, r2_l);
                r1.borrow_mut().right = MERGE(r1_r, r2_r);

                Some(r1)
            }
        };

        MERGE(root1, root2)
    }

    ///
    /// DFS, iteration, reuse root1
    ///
    fn dfs_iter_reuse(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Ensure that r1 is not None
        if root1.is_none() {
            return root2;
        }

        let mut stack = vec![(root1.clone(), root2)];
        while let Some((root1, root2)) = stack.pop() {
            match (root1, root2) {
                (Some(r1), Some(r2)) => {
                    let mut r1 = r1.borrow_mut();
                    let mut r2 = r2.borrow_mut();
                    r1.val += r2.val;

                    if r1.left.is_none() {
                        r1.left = r2.left.take();
                    } else {
                        stack.push((r1.left.clone(), r2.left.clone()));
                    }

                    if r1.right.is_none() {
                        r1.right = r2.right.take();
                    } else {
                        stack.push((r1.right.clone(), r2.right.clone()));
                    }
                }
                _ => {}
            }
        }

        root1
    }

    ///
    /// BFS, iteration, create a new node
    ///
    fn bfs_iter_create_new(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = None;
        let mut queue = VecDeque::from([(None, root1, root2, false)]);

        while let Some((parent, r1, r2, is_left)) = queue.pop_front() {
            let node = match (r1, r2) {
                (None, r2) => r2,
                (r1, None) => r1,
                (Some(r1), Some(r2)) => {
                    let mut r1 = r1.borrow_mut();
                    let mut r2 = r2.borrow_mut();

                    let new = Some(Rc::new(RefCell::new(TreeNode::new(r1.val + r2.val))));
                    if r1.left.is_some() || r2.left.is_some() {
                        queue.push_back((new.clone(), r1.left.take(), r2.left.take(), true));
                    }
                    if r1.right.is_some() || r2.right.is_some() {
                        queue.push_back((new.clone(), r1.right.take(), r2.right.take(), false));
                    }

                    new
                }
            };

            if let Some(p) = parent {
                if is_left {
                    p.borrow_mut().left = node;
                } else {
                    p.borrow_mut().right = node;
                }
            } else {
                root = node;
            }
        }

        root
    }

    ///
    /// BFS, iteration, reuse root1
    ///
    fn bfs_iter_reuse(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // Ensure that r1 is not None
        if root1.is_none() {
            return root2;
        }

        let mut queue = VecDeque::from([(root1.clone(), root2)]);
        while let Some((r1, r2)) = queue.pop_front() {
            match (r1, r2) {
                (Some(r1), Some(r2)) => {
                    let mut r1 = r1.borrow_mut();
                    let mut r2 = r2.borrow_mut();

                    r1.val += r2.val;

                    if r1.left.is_none() {
                        r1.left = r2.left.take();
                    } else {
                        queue.push_back((r1.left.clone(), r2.left.clone()));
                    }

                    if r1.right.is_none() {
                        r1.right = r2.right.take();
                    } else {
                        queue.push_back((r1.right.clone(), r2.right.clone()));
                    }
                }
                _ => {}
            }
        }

        root1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
