//Given the root of a binary search tree (BST) with duplicates, return all the
//mode(s) (i.e., the most frequently occurred element) in it.
//
// If the tree has more than one mode, return them in any order.
//
// Assume a BST is defined as follows:
//
//
// The left subtree of a node contains only nodes with keys less than or equal
//to the node's key.
// The right subtree of a node contains only nodes with keys greater than or
//equal to the node's key.
// Both the left and right subtrees must also be binary search trees.
//
//
//
// Example 1:
//
//
//Input: root = [1,null,2,2]
//Output: [2]
//
//
// Example 2:
//
//
//Input: root = [0]
//Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁴].
// -10⁵ <= Node.val <= 10⁵
//
//
//
//Follow up: Could you do that without using any extra space? (Assume that the
//implicit stack space incurred due to recursion does not count).
//
// Related Topics Tree Depth-First Search Binary Search Tree Binary Tree 👍 3800
// 👎 769

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::usize;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Self::use_hashmap_recur(root)
        //Self::use_hashmap_iter(root)

        //Self::in_order_traversal_recur(root)
        //Self::in_order_traversal_iter(root)

        //Self::morris_in_order_iter_1(root)
        Self::morris_in_order_iter_2(root)
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    ///
    fn use_hashmap_recur(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        const PREORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut HashMap<i32, usize>) =
            |root, counter| {
                if let Some(curr) = root {
                    let curr_val = curr.borrow().val;
                    counter
                        .entry(curr_val)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    PREORDER(curr.borrow_mut().left.take(), counter);
                    PREORDER(curr.borrow_mut().right.take(), counter);
                }
            };

        PREORDER(root, &mut map);

        let max_freq = map.values().max().map(|v| *v).unwrap_or_default();
        map.into_iter()
            .filter_map(|(k, v)| {
                if v == max_freq {
                    return Some(k);
                }
                None
            })
            .collect()
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    ///
    fn use_hashmap_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut map = HashMap::new();
        if let Some(root) = root {
            let mut stack = vec![root];

            while let Some(curr) = stack.pop() {
                let curr_val = curr.borrow().val;
                map.entry(curr_val)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);

                if let Some(right) = curr.borrow_mut().right.take() {
                    stack.push(right);
                }
                if let Some(left) = curr.borrow_mut().left.take() {
                    stack.push(left);
                }
            }
        }

        let max_freq = map.values().max().map(|v| *v).unwrap_or_default();
        map.into_iter()
            .filter_map(|(k, v)| {
                if v == max_freq {
                    return Some(k);
                }
                None
            })
            .collect()
    }

    ///       val: The value of the node being traversed
    ///  curr_val: Value being processed
    /// curr_freq: The frequency of occurrences of values being processed
    ///  max_freq: Maximum frequency of occurrence
    ///       res: result
    fn update(
        val: i32,
        curr_val: &mut i32,
        curr_freq: &mut usize,
        max_freq: &mut usize,
        res: &mut Vec<i32>,
    ) {
        if val == *curr_val {
            *curr_freq += 1;
        } else {
            *curr_val = val;
            *curr_freq = 1;
        }
        if *curr_freq > *max_freq {
            res.clear();
            *max_freq = *curr_freq;
        }
        if *curr_freq == *max_freq {
            res.push(val);
        }
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    ///
    fn in_order_traversal_recur(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut curr_val = i32::MIN;
        let mut curr_freq = 0;
        let mut max_freq = 0;
        const INORDER: fn(
            Option<Rc<RefCell<TreeNode>>>,
            &mut i32,
            &mut usize,
            &mut usize,
            &mut Vec<i32>,
        ) = |root, curr_val, curr_freq, max_freq, res| {
            if let Some(curr) = root {
                INORDER(
                    curr.borrow_mut().left.take(),
                    curr_val,
                    curr_freq,
                    max_freq,
                    res,
                );

                let val = curr.borrow().val;
                Solution::update(val, curr_val, curr_freq, max_freq, res);

                INORDER(
                    curr.borrow_mut().right.take(),
                    curr_val,
                    curr_freq,
                    max_freq,
                    res,
                );
            }
        };

        INORDER(root, &mut curr_val, &mut curr_freq, &mut max_freq, &mut res);

        res
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(n)
    ///
    fn in_order_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        if let Some(root) = root {
            let mut curr_val = i32::MIN;
            let mut curr_freq = 0;
            let mut max_freq = 0;
            let mut stack = vec![Ok(root)];

            while let Some(curr) = stack.pop() {
                match curr {
                    Ok(node) => {
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push(Ok(right));
                        }
                        stack.push(Err(node.borrow().val));
                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push(Ok(left));
                        }
                    }
                    Err(val) => {
                        Solution::update(
                            val,
                            &mut curr_val,
                            &mut curr_freq,
                            &mut max_freq,
                            &mut res,
                        );
                    }
                }
            }
        }

        res
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(1)
    ///
    fn morris_in_order_iter_1(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut curr_val = i32::MIN;
        let mut curr_freq = 0;
        let mut max_freq = 0;

        while let Some(curr) = root {
            let val = curr.borrow().val;
            let left = curr.borrow().left.clone();

            if left.is_some() {
                let mut prev_node = left.clone();

                while let Some(ref prev) = prev_node {
                    let right = prev.borrow().right.clone();
                    if right.is_none() || right == Some(curr.clone()) {
                        break;
                    } else {
                        prev_node = right;
                    }
                }

                match prev_node {
                    None => break, // this is mark code
                    Some(prev) => {
                        let mut prev = prev.borrow_mut();

                        if let Some(_) = prev.right.take() {
                            Solution::update(
                                val,
                                &mut curr_val,
                                &mut curr_freq,
                                &mut max_freq,
                                &mut res,
                            );
                            root = curr.borrow().right.clone();
                        } else {
                            prev.right = Some(curr);
                            root = left;
                        }
                    }
                }
            } else {
                Solution::update(val, &mut curr_val, &mut curr_freq, &mut max_freq, &mut res);
                root = curr.borrow().right.clone();
            };
        }

        res
    }

    ///
    /// Time complexity: O(n)
    /// Space complexity: O(1)
    ///
    fn morris_in_order_iter_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut curr_val = i32::MIN;
        let mut curr_freq = 0;
        let mut max_freq = 0;

        while let Some(curr) = root {
            let left = curr.borrow().left.clone();
            let val = curr.borrow().val;

            if left.is_none() {
                Solution::update(val, &mut curr_val, &mut curr_freq, &mut max_freq, &mut res);
                root = curr.borrow().right.clone();
                continue;
            }

            let mut prev_node = left.clone();
            while let Some(ref prev) = prev_node {
                let right = prev.borrow().right.clone();
                if right.is_none() || right == Some(curr.clone()) {
                    break;
                } else {
                    prev_node = right;
                }
            }

            if let Some(prev) = prev_node {
                let mut prev = prev.borrow_mut();

                if let Some(_) = prev.right.take() {
                    Solution::update(val, &mut curr_val, &mut curr_freq, &mut max_freq, &mut res);
                    root = curr.borrow().right.clone();
                } else {
                    prev.right = Some(curr);
                    root = left;
                }
            } else {
                // here is mark code
                //root = None;
                break;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
