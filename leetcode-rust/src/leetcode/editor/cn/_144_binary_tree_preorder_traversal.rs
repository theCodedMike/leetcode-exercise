//Given the root of a binary tree, return the preorder traversal of its nodes'
//values.
//
//
// Example 1:
//
//
//Input: root = [1,null,2,3]
//Output: [1,2,3]
//
//
// Example 2:
//
//
//Input: root = []
//Output: []
//
//
// Example 3:
//
//
//Input: root = [1]
//Output: [1]
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
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
//
// Related Topics Stack Tree Depth-First Search Binary Tree 👍 7384 👎 192

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        //Self::recursion_impl(root)

        //Self::iteration_impl_1(root)
        //Self::iteration_impl_2(root)
        //Self::iteration_impl_3(root)
        //Self::iteration_impl_4(root)

        Self::morris_impl(root)
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n), the space is taken up by the recursive call stack.
    fn recursion_impl(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        const PREORDER: fn(Option<Rc<RefCell<TreeNode>>>, &mut Vec<i32>) = |root, res| {
            if let Some(node) = root {
                res.push(node.borrow().val);

                PREORDER(node.borrow_mut().left.take(), res);
                PREORDER(node.borrow_mut().right.take(), res);
            }
        };

        PREORDER(root, &mut res);

        res
    }

    /// 这种写法可以作为前序遍历或后序遍历的通用写法
    fn iteration_impl_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(node) = root {
            let mut stack = vec![node];

            while let Some(curr) = stack.pop() {
                res.push(curr.borrow().val);

                if let Some(right) = curr.borrow_mut().right.take() {
                    stack.push(right);
                }
                if let Some(left) = curr.borrow_mut().left.take() {
                    stack.push(left);
                }
            }
        }

        res
    }

    fn iteration_impl_2(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(curr) = root {
                res.push(curr.borrow().val);
                root = curr.borrow_mut().left.take();
                stack.push(curr);
            }

            if let Some(curr) = stack.pop() {
                root = curr.borrow_mut().right.take();
            }
        }

        res
    }

    fn iteration_impl_3(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        let mut stack = vec![];
        while root.is_some() || !stack.is_empty() {
            match root {
                Some(curr) => {
                    res.push(curr.borrow().val);
                    root = curr.borrow_mut().left.take();
                    stack.push(curr);
                }
                None => {
                    if let Some(curr) = stack.pop() {
                        root = curr.borrow_mut().right.take();
                    }
                }
            }
        }

        res
    }

    /// 这种写法可以作为通用写法
    fn iteration_impl_4(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        if let Some(root) = root {
            let mut stack = vec![Ok(root)];

            while let Some(curr) = stack.pop() {
                match curr {
                    Ok(node) => {
                        if let Some(right) = node.borrow_mut().right.take() {
                            stack.push(Ok(right));
                        }
                        if let Some(left) = node.borrow_mut().left.take() {
                            stack.push(Ok(left));
                        }
                        stack.push(Err(node.borrow().val));
                    }
                    Err(val) => res.push(val),
                }
            }
        }

        res
    }

    fn morris_impl(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];

        while let Some(curr) = root {
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
                    None => break,
                    Some(prev) => {
                        let mut prev = prev.borrow_mut();
                        if let Some(_) = prev.right.take() {
                            root = curr.borrow().right.clone();
                        } else {
                            res.push(curr.borrow().val);
                            prev.right = Some(curr);
                            root = left;
                        }
                    }
                }
            } else {
                res.push(curr.borrow().val);
                root = curr.borrow().right.clone();
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
