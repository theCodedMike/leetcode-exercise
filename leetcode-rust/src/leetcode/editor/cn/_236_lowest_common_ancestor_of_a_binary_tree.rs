//Given a binary tree, find the lowest common ancestor (LCA) of two given nodes
//in the tree.
//
// According to the definition of LCA on Wikipedia: “The lowest common ancestor
//is defined between two nodes p and q as the lowest node in T that has both p
//and q as descendants (where we allow a node to be a descendant of itself).”
//
//
// Example 1:
//
//
//Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 1
//Output: 3
//Explanation: The LCA of nodes 5 and 1 is 3.
//
//
// Example 2:
//
//
//Input: root = [3,5,1,6,2,0,8,null,null,7,4], p = 5, q = 4
//Output: 5
//Explanation: The LCA of nodes 5 and 4 is 5, since a node can be a descendant
//of itself according to the LCA definition.
//
//
// Example 3:
//
//
//Input: root = [1,2], p = 1, q = 2
//Output: 1
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
// p and q will exist in the tree.
//
//
// Related Topics Tree Depth-First Search Binary Tree 👍 15808 👎 376

#![allow(dead_code)]
#![allow(unused)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::dfs_recur(root, p, q)

        Self::store_parent_node(root, p, q)
    }
    fn dfs_recur(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const POSTORDER: fn(
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
            Option<Rc<RefCell<TreeNode>>>,
        ) -> Option<Rc<RefCell<TreeNode>>> = |root, p, q| match root {
            None => None,
            Some(curr) => {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();
                let curr = Some(curr);
                if curr == p || curr == q {
                    return curr;
                }

                let l_res = POSTORDER(left, p.clone(), q.clone());
                let r_res = POSTORDER(right, p.clone(), q.clone());

                if l_res.is_some() && r_res.is_some() {
                    return curr;
                }

                return if l_res.is_some() { l_res } else { r_res };
            }
        };

        POSTORDER(root, p, q)
    }

    fn store_parent_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        mut p: Option<Rc<RefCell<TreeNode>>>,
        mut q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map = HashMap::new();

        if let Some(root) = root {
            map.insert(root.borrow().val, (None, false));

            const BUILD: fn(
                Rc<RefCell<TreeNode>>,
                &mut HashMap<i32, (Option<Rc<RefCell<TreeNode>>>, bool)>,
            ) = |root, map| {
                if let Some(left) = root.borrow_mut().left.take() {
                    map.insert(left.borrow().val, (Some(root.clone()), false));
                    BUILD(left, map);
                }
                if let Some(right) = root.borrow_mut().right.take() {
                    map.insert(right.borrow().val, (Some(root.clone()), false));
                    BUILD(right, map);
                }
            };

            BUILD(root, &mut map);
        }

        while let Some(ref curr) = p {
            let p_val = curr.borrow().val;
            if let Some((parent, is_visit)) = map.get_mut(&p_val) {
                *is_visit = true;
                p = parent.clone();
            }
        }

        while let Some(ref curr) = q {
            let q_val = curr.borrow().val;
            let (parent, is_visit) = &map[&q_val];
            if *is_visit {
                return q;
            }
            q = parent.clone();
        }

        None
    }
}
//leetcode submit region end(Prohibit modification and deletion)
