//Given the root of a binary tree, imagine yourself standing on the right side
//of it, return the values of the nodes you can see ordered from top to bottom.
//
//
// Example 1:
//
//
//Input: root = [1,2,3,null,5,null,4]
//Output: [1,3,4]
//
//
// Example 2:
//
//
//Input: root = [1,null,3]
//Output: [1,3]
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
// Related Topics Tree Depth-First Search Breadth-First Search Binary Tree 👍 11
//058 👎 724

#![allow(dead_code)]
pub struct Solution;
use crate::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Self::dfs_recursion(root)
        //Self::dfs_iteration_1(root)
        //Self::dfs_iteration_2(root)
        //Self::dfs_iteration_3(root)
        //Self::bfs_iteration_1(root)
        Self::bfs_iteration_2(root)
    }

    fn dfs_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        const RECURSION_IMPL: fn(Option<Rc<RefCell<TreeNode>>>, usize, &mut Vec<i32>) =
            |root, level, res| {
                if let Some(curr) = root {
                    if level == res.len() {
                        res.push(curr.borrow().val);
                    }

                    // go Right
                    RECURSION_IMPL(curr.borrow_mut().right.take(), level + 1, res);
                    // go Left
                    RECURSION_IMPL(curr.borrow_mut().left.take(), level + 1, res);
                }
            };

        RECURSION_IMPL(root, 0, &mut res);

        res
    }

    fn dfs_iteration_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut root = (root, 0);
        let mut stack = vec![];
        while root.0.is_some() || !stack.is_empty() {
            match root.0 {
                Some(curr) => {
                    let curr_level = root.1;
                    if res.len() == curr_level {
                        res.push(curr.borrow().val);
                    }
                    root = (curr.borrow_mut().right.take(), curr_level + 1);
                    stack.push((curr, curr_level));
                }
                None => {
                    if let Some((curr, curr_level)) = stack.pop() {
                        root = (curr.borrow_mut().left.take(), curr_level + 1);
                    }
                }
            }
        }

        res
    }

    fn dfs_iteration_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut root = (root, 0);
        let mut stack = vec![];
        while root.0.is_some() || !stack.is_empty() {
            while let Some(curr) = root.0 {
                let curr_level = root.1;
                if res.len() == curr_level {
                    res.push(curr.borrow().val);
                }
                root = (curr.borrow_mut().right.take(), curr_level + 1);
                stack.push((curr, curr_level));
            }

            if let Some((curr, curr_level)) = stack.pop() {
                root = (curr.borrow_mut().left.take(), curr_level + 1);
            }
        }

        res
    }

    fn dfs_iteration_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![Ok((root, 0))];

            while let Some(root) = stack.pop() {
                match root {
                    Ok((curr, curr_level)) => {
                        if let Some(left) = curr.borrow_mut().left.take() {
                            stack.push(Ok((left, curr_level + 1)));
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            stack.push(Ok((right, curr_level + 1)));
                        }
                        stack.push(Err((curr.borrow().val, curr_level)));
                    }
                    Err((val, curr_level)) => {
                        if res.len() == curr_level {
                            res.push(val);
                        }
                    }
                }
            }
        }

        res
    }

    fn bfs_iteration_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([root]);

            while !queue.is_empty() {
                let level_len = queue.len();
                for i in 1..=level_len {
                    if let Some(curr) = queue.pop_front() {
                        // if you enqueue left node first, here should be i == level_len
                        // if you enqueue right node first, here should be i == 0
                        if i == level_len {
                            res.push(curr.borrow().val);
                        }
                        if let Some(left) = curr.borrow_mut().left.take() {
                            queue.push_back(left);
                        }
                        if let Some(right) = curr.borrow_mut().right.take() {
                            queue.push_back(right);
                        }
                    }
                }
            }
        }

        res
    }

    fn bfs_iteration_2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut queue = VecDeque::from([(root, 0)]);

            while let Some((curr, curr_level)) = queue.pop_front() {
                if curr_level == res.len() {
                    res.push(curr.borrow().val);
                }
                if let Some(right) = curr.borrow_mut().right.take() {
                    queue.push_back((right, curr_level + 1));
                }
                if let Some(left) = curr.borrow_mut().left.take() {
                    queue.push_back((left, curr_level + 1));
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
