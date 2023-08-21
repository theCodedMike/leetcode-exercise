//Given the root of a binary tree, return the bottom-up level order traversal
//of its nodes' values. (i.e., from left to right, level by level from leaf to root)
//.
//
//
// Example 1:
//
//
//Input: root = [3,9,20,null,null,15,7]
//Output: [[15,7],[9,20],[3]]
//
//
// Example 2:
//
//
//Input: root = [1]
//Output: [[1]]
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
// The number of nodes in the tree is in the range [0, 2000].
// -1000 <= Node.val <= 1000
//
//
// Related Topics Tree Breadth-First Search Binary Tree 👍 4536 👎 315

#![allow(dead_code)]

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
}

//leetcode submit region begin(Prohibit modification and deletion)

use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;
impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        //Self::level_order1(root)
        Self::level_order2(root)
        //Self::level_order3(root)
    }

    fn level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        match root {
            None => {}
            Some(curr) => {
                let mut queue = VecDeque::new();
                queue.push_back((curr, 0));

                while let Some((curr, level)) = queue.pop_front() {
                    let val = curr.borrow().val;
                    match res.get_mut(level) {
                        None => {
                            res.push(vec![val]);
                        }
                        Some(vec) => {
                            vec.push(val);
                        }
                    }

                    if let Some(left) = curr.borrow_mut().left.take() {
                        queue.push_back((left, level + 1));
                    }
                    if let Some(right) = curr.borrow_mut().right.take() {
                        queue.push_back((right, level + 1));
                    }
                }

                res.reverse()
            }
        }

        res
    }

    fn level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        match root {
            None => {}
            Some(curr) => {
                let mut queue = VecDeque::new();
                queue.push_back((curr, 0));
                let mut prev_leval = -1;

                while let Some((curr, level)) = queue.pop_front() {
                    let val = curr.borrow().val;
                    if prev_leval != level {
                        res.insert(0, vec![val]);
                    } else {
                        res[0].push(val);
                    }

                    prev_leval = level;
                    if let Some(left) = curr.borrow_mut().left.take() {
                        queue.push_back((left, level + 1));
                    }
                    if let Some(right) = curr.borrow_mut().right.take() {
                        queue.push_back((right, level + 1));
                    }
                }
            }
        }

        res
    }

    fn level_order3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = BTreeMap::new();

        match root {
            None => {}
            Some(curr) => {
                let mut queue = VecDeque::new();
                queue.push_back((curr, 0));

                while let Some((curr, level)) = queue.pop_front() {
                    let val = curr.borrow().val;
                    res.entry(level)
                        .and_modify(|vec: &mut Vec<i32>| vec.push(val))
                        .or_insert(vec![val]);

                    if let Some(left) = curr.borrow_mut().left.take() {
                        queue.push_back((left, level + 1));
                    }
                    if let Some(right) = curr.borrow_mut().right.take() {
                        queue.push_back((right, level + 1));
                    }
                }
            }
        }

        res.values().rev().cloned().collect::<Vec<_>>()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
