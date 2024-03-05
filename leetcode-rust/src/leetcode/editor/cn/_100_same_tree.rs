//Given the roots of two binary trees p and q, write a function to check if
//they are the same or not.
//
// Two binary trees are considered the same if they are structurally identical,
//and the nodes have the same value.
//
//
// Example 1:
//
//
//Input: p = [1,2,3], q = [1,2,3]
//Output: true
//
//
// Example 2:
//
//
//Input: p = [1,2], q = [1,null,2]
//Output: false
//
//
// Example 3:
//
//
//Input: p = [1,2,1], q = [1,1,2]
//Output: false
//
//
//
// Constraints:
//
//
// The number of nodes in both trees is in the range [0, 100].
// -10⁴ <= Node.val <= 10⁴
//
//
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 95
//90 👎 192

#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        //Self::dfs_recur(p, q)
        //Self::dfs_iter(p, q)

        Self::bfs_iter(p, q)
    }

    fn dfs_recur(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        const COMPARE: fn(Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) -> bool =
            |p, q| match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    if p.borrow().val != q.borrow().val {
                        return false;
                    }

                    COMPARE(p.borrow_mut().left.take(), q.borrow_mut().left.take())
                        && COMPARE(p.borrow_mut().right.take(), q.borrow_mut().right.take())
                }
                _ => false,
            };

        COMPARE(p, q)
    }

    fn dfs_iter(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![(p, q)];

        while let Some((p, q)) = stack.pop() {
            match (p, q) {
                (None, None) => {}
                (Some(p), Some(q)) => {
                    if p.borrow().val != q.borrow().val {
                        return false;
                    }

                    stack.push((p.borrow_mut().right.take(), q.borrow_mut().right.take()));
                    stack.push((p.borrow_mut().left.take(), q.borrow_mut().left.take()));
                }
                _ => return false,
            }
        }

        true
    }

    fn bfs_iter(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut queue = VecDeque::from([(p, q)]);

        while let Some((p, q)) = queue.pop_front() {
            match (p, q) {
                (None, None) => {}
                (Some(p), Some(q)) => {
                    if p.borrow().val != q.borrow().val {
                        return false;
                    }

                    queue.push_back((p.borrow_mut().left.take(), q.borrow_mut().left.take()));
                    queue.push_back((p.borrow_mut().right.take(), q.borrow_mut().right.take()));
                }
                _ => return false,
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
