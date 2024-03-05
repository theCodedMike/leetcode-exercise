//Given a binary search tree (BST), find the lowest common ancestor (LCA) node
//of two given nodes in the BST.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor
//is defined between two nodes p and q as the lowest node in T that has both p
//and q as descendants (where we allow a node to be a descendant of itself).”
//
//
// Example 1:
//
//
//Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
//Output: 6
//Explanation: The LCA of nodes 2 and 8 is 6.
//
//
// Example 2:
//
//
//Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
//Output: 2
//Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant
//of itself according to the LCA definition.
//
//
// Example 3:
//
//
//Input: root = [2,1], p = 2, q = 1
//Output: 2
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [2, 10⁵].
// -10⁹ <= Node.val <= 10⁹
// All Node.val are unique.
// p != q
// p and q will exist in the BST.
//
//
// Related Topics 树 深度优先搜索 二叉搜索树 二叉树 👍 1191 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::two_traversal_recur(root, p, q)
        //Self::two_traversal_iter(root, p, q)

        //Self::one_traversal_recur(root, p, q)
        Self::one_traversal_iter(root, p, q)
    }

    fn two_traversal_recur(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        const TRAVERSAL: fn(
            Option<Rc<RefCell<TreeNode>>>,
            i32,
            &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        ) = |root, target, path| {
            if let Some(curr) = root {
                let curr_val = curr.borrow().val;
                path.push(Some(curr.clone()));

                if target > curr_val {
                    TRAVERSAL(curr.borrow().right.clone(), target, path);
                } else if target < curr_val {
                    TRAVERSAL(curr.borrow().left.clone(), target, path);
                } else {
                    return;
                }
            }
        };

        let mut p_path = vec![];
        TRAVERSAL(root.clone(), p, &mut p_path);
        let mut q_path = vec![];
        TRAVERSAL(root, q, &mut q_path);

        let len = std::cmp::min(p_path.len(), q_path.len());
        for i in (0..len).rev() {
            if p_path[i] == q_path[i] {
                return p_path[i].clone();
            }
        }

        None
    }

    fn two_traversal_iter(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;
        let traversal = |mut root: Option<Rc<RefCell<TreeNode>>>, target: i32| {
            let mut path = vec![];
            while let Some(curr) = root {
                let curr_val = curr.borrow().val;
                path.push(Some(curr.clone()));

                if target > curr_val {
                    root = curr.borrow().right.clone();
                } else if target < curr_val {
                    root = curr.borrow().left.clone();
                } else {
                    break;
                }
            }
            path
        };

        let p_path = traversal(root.clone(), p);
        let q_path = traversal(root, q);

        let len = std::cmp::min(p_path.len(), q_path.len());
        for i in (0..len).rev() {
            if p_path[i] == q_path[i] {
                return p_path[i].clone();
            }
        }

        None
    }

    fn one_traversal_recur(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        const TRAVERSAL: fn(
            Option<Rc<RefCell<TreeNode>>>,
            i32,
            i32,
        ) -> Option<Rc<RefCell<TreeNode>>> = |root, p, q| match root {
            None => None,
            Some(curr) => {
                let curr_val = curr.borrow().val;

                if p < curr_val && q < curr_val {
                    TRAVERSAL(curr.borrow().left.clone(), p, q)
                } else if p > curr_val && q > curr_val {
                    TRAVERSAL(curr.borrow().right.clone(), p, q)
                } else {
                    Some(curr)
                }
            }
        };

        TRAVERSAL(root, p, q)
    }

    fn one_traversal_iter(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        let traversal = |mut root: Option<Rc<RefCell<TreeNode>>>| {
            while let Some(curr) = root {
                let curr_val = curr.borrow().val;

                if p < curr_val && q < curr_val {
                    root = curr.borrow().left.clone();
                } else if p > curr_val && q > curr_val {
                    root = curr.borrow().right.clone();
                } else {
                    return Some(curr);
                }
            }

            None
        };

        traversal(root)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
