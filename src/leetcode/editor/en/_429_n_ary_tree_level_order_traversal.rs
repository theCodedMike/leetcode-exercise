//Given an n-ary tree, return the level order traversal of its nodes' values.
//
// Nary-Tree input serialization is represented in their level order traversal,
//each group of children is separated by the null value (See examples).
//
//
// Example 1:
//
//
//
//
//Input: root = [1,null,3,2,4,null,5,6]
//Output: [[1],[3,2,4],[5,6]]
//
//
// Example 2:
//
//
//
//
//Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,
//null,12,null,13,null,null,14]
//Output: [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]
//
//
//
// Constraints:
//
//
// The height of the n-ary tree is less than or equal to 1000
// The total number of nodes is between [0, 10⁴]
//
//
// Related Topics Tree Breadth-First Search 👍 3555 👎 135

#![allow(dead_code)]

pub struct Solution;
use crate::Node;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        //Self::dfs_recur_pre_order(root)
        //Self::dfs_iter_pre_order_1(root)
        Self::dfs_iter_pre_order_2(root)
        //Self::dfs_iter_pre_order_3(root)
        //Self::bfs_iter_1(root)
        //Self::bfs_iter_2(root)
    }

    ///
    /// DFS - Recursion(Pre-Order)
    ///
    fn dfs_recur_pre_order(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        const PRE_ORDER: fn(Option<Rc<RefCell<Node>>>, usize, &mut Vec<Vec<i32>>) =
            |root, level, res| {
                if let Some(curr) = root {
                    if level == res.len() {
                        res.push(vec![]);
                    }
                    res[level].push(curr.borrow().val);

                    if let Some(children) = curr.borrow_mut().children.take() {
                        for child in children {
                            PRE_ORDER(child, level + 1, res);
                        }
                    }
                }
            };

        PRE_ORDER(root, 0, &mut res);

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_1(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut root = (root, 0);
        let mut stack = vec![];
        while root.0.is_some() || !stack.is_empty() {
            while let Some(curr) = root.0 {
                let level = root.1;
                if level == res.len() {
                    res.push(vec![]);
                }
                res[level].push(curr.borrow().val);

                root = if let Some(ref mut children) = curr.borrow_mut().children {
                    (children[0].take(), level + 1)
                } else {
                    (None, level + 1)
                };
                stack.push((curr, level));
            }

            if let Some((curr, level)) = stack.pop() {
                root = if let Some(ref mut children) = curr.borrow_mut().children {
                    if children.len() <= 1 {
                        (None, level + 1)
                    } else {
                        let right = children.remove(1);
                        if children.len() > 1 {
                            stack.push((curr.clone(), level));
                        }
                        (right, level + 1)
                    }
                } else {
                    (None, level + 1)
                };
            };
        }

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_2(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let mut root = (root, 0);
        let mut stack = vec![];
        while root.0.is_some() || !stack.is_empty() {
            if let Some(curr) = root.0 {
                let level = root.1;
                if level == res.len() {
                    res.push(vec![]);
                }
                res[level].push(curr.borrow().val);

                root = (None, level + 1);
                if let Some(ref mut children) = curr.borrow_mut().children {
                    root.0 = children[0].take();
                }
                stack.push((curr, level));
            } else {
                if let Some((curr, level)) = stack.pop() {
                    root = (None, level + 1);
                    if let Some(ref mut children) = curr.borrow_mut().children {
                        if children.len() > 1 {
                            root.0 = children.remove(1);
                            if children.len() > 1 {
                                stack.push((curr.clone(), level));
                            }
                        }
                    };
                };
            }
        }

        res
    }

    ///
    /// DFS - Iteration(Pre-Order)
    ///
    fn dfs_iter_pre_order_3(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![(Ok(root), 0)];

            while let Some((curr, level)) = stack.pop() {
                match curr {
                    Ok(node) => {
                        if let Some(mut children) = node.borrow_mut().children.take() {
                            for i in (0..children.len()).rev() {
                                if let Some(child) = children[i].take() {
                                    stack.push((Ok(child), level + 1));
                                }
                            }
                        }
                        stack.push((Err(node.borrow().val), level));
                    }
                    Err(val) => {
                        if level == res.len() {
                            res.push(vec![]);
                        }
                        res[level].push(val);
                    }
                }
            }
        }

        res
    }

    ///
    /// BFS - Iteration(Level Order)
    ///
    fn bfs_iter_1(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();
                let mut level_vec = vec![];

                for _ in 0..level_len {
                    if let Some(curr) = queue.pop_front() {
                        level_vec.push(curr.borrow().val);
                        if let Some(children) = curr.borrow_mut().children.take() {
                            for child in children {
                                if let Some(child) = child {
                                    queue.push_back(child);
                                }
                            }
                        }
                    }
                }

                res.push(level_vec);
            }
        }

        res
    }

    ///
    /// BFS - Iteration(Level Order)
    ///
    fn bfs_iter_2(root: Option<Rc<RefCell<Node>>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);

            while let Some((curr, level)) = queue.pop_front() {
                if level == res.len() {
                    res.push(vec![]);
                }
                res[level].push(curr.borrow().val);

                if let Some(children) = curr.borrow_mut().children.take() {
                    for child in children {
                        if let Some(child) = child {
                            queue.push_back((child, level + 1));
                        }
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
