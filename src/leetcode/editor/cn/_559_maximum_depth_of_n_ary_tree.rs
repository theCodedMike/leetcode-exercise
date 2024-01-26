//Given a n-ary tree, find its maximum depth.
//
// The maximum depth is the number of nodes along the longest path from the
//root node down to the farthest leaf node.
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
//Output: 3
//
//
// Example 2:
//
//
//
//
//Input: root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,
//null,12,null,13,null,null,14]
//Output: 5
//
//
//
// Constraints:
//
//
// The total number of nodes is in the range [0, 10⁴].
// The depth of the n-ary tree is less than or equal to 1000.
//
//
// Related Topics 树 深度优先搜索 广度优先搜索 👍 380 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::n_ary_tree::Node;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<Node>>>) -> i32 {
        //Self::dfs_recur(root)
        //Self::dfs_iter(root)

        //Self::bfs_iter_1(root)
        Self::bfs_iter_2(root)
    }

    fn dfs_recur(root: Option<Rc<RefCell<Node>>>) -> i32 {
        const DFS: fn(Option<Rc<RefCell<Node>>>) -> i32 = |root| match root {
            None => 0,
            Some(node) => {
                let mut max_depth = 0;

                let children = node.borrow_mut().children.take();
                if let Some(nodes) = children {
                    for node in nodes {
                        max_depth = std::cmp::max(max_depth, DFS(node));
                    }
                }

                max_depth + 1
            }
        };

        DFS(root)
    }

    fn dfs_iter(root: Option<Rc<RefCell<Node>>>) -> i32 {
        let mut max_depth = 0;

        if let Some(root) = root {
            let mut stack = vec![(root, 1)];
            while let Some((curr, depth)) = stack.pop() {
                max_depth = std::cmp::max(max_depth, depth);

                let children = curr.borrow_mut().children.take();
                if let Some(nodes) = children {
                    for node in nodes {
                        if let Some(node) = node {
                            stack.push((node, depth + 1));
                        }
                    }
                }
            }
        }

        max_depth
    }

    fn bfs_iter_1(root: Option<Rc<RefCell<Node>>>) -> i32 {
        let mut max_depth = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 1)]);
            while let Some((curr, depth)) = queue.pop_front() {
                max_depth = std::cmp::max(max_depth, depth);

                let children = curr.borrow_mut().children.take();
                if let Some(nodes) = children {
                    for node in nodes {
                        if let Some(node) = node {
                            queue.push_back((node, depth + 1));
                        }
                    }
                }
            }
        }

        max_depth
    }

    fn bfs_iter_2(root: Option<Rc<RefCell<Node>>>) -> i32 {
        let mut max_depth = 0;

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);
            while !queue.is_empty() {
                let len = queue.len();

                for _ in 0..len {
                    if let Some(curr) = queue.pop_front() {
                        let children = curr.borrow_mut().children.take();
                        if let Some(nodes) = children {
                            for node in nodes {
                                if let Some(node) = node {
                                    queue.push_back(node);
                                }
                            }
                        }
                    }
                }

                max_depth += 1;
            }
        }

        max_depth
    }
}

//leetcode submit region end(Prohibit modification and deletion)
