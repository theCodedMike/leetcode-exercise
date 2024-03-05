//Given a root node reference of a BST and a key, delete the node with the
//given key in the BST. Return the root node reference (possibly updated) of the BST.
//
// Basically, the deletion can be divided into two stages:
//
//
// Search for a node to remove.
// If the node is found, delete the node.
//
//
//
// Example 1:
//
//
//Input: root = [5,3,6,2,4,null,7], key = 3
//Output: [5,4,6,2,null,null,7]
//Explanation: Given key to delete is 3. So we find the node with value 3 and
//delete it.
//One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
//Please notice that another valid answer is [5,2,6,null,4,null,7] and it's
//also accepted.
//
//
//
// Example 2:
//
//
//Input: root = [5,3,6,2,4,null,7], key = 0
//Output: [5,3,6,2,4,null,7]
//Explanation: The tree does not contain a node with value = 0.
//
//
// Example 3:
//
//
//Input: root = [], key = 0
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [0, 10⁴].
// -10⁵ <= Node.val <= 10⁵
// Each node has a unique value.
// root is a valid binary search tree.
// -10⁵ <= key <= 10⁵
//
//
//
// Follow up: Could you solve it with time complexity O(height of tree)?
//
// Related Topics 树 二叉搜索树 二叉树 👍 1297 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::recur_helper(root, key)

        Self::iter_helper(root, key)
    }

    fn recur_helper(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const DELETE: fn(
            &mut Option<Rc<RefCell<TreeNode>>>,
            i32,
            Option<Rc<RefCell<TreeNode>>>,
            bool,
        ) = |root, key, mut parent, is_left| {
            if let Some(curr) = root {
                let curr_val = curr.borrow().val;

                if curr_val == key {
                    let right = curr.borrow_mut().right.take();
                    let left = curr.borrow_mut().left.take();

                    let child = if right.is_some() {
                        let mut leftmost = right.clone();
                        while let Some(ref curr) = leftmost {
                            let left = curr.borrow().left.clone();
                            if left.is_none() {
                                break;
                            } else {
                                leftmost = left;
                            }
                        }

                        if let Some(curr) = leftmost {
                            curr.borrow_mut().left = left;
                        }

                        right
                    } else {
                        left
                    };

                    if let Some(p) = parent {
                        if is_left {
                            p.borrow_mut().left = child;
                        } else {
                            p.borrow_mut().right = child;
                        }
                    } else {
                        *root = child;
                    }
                } else {
                    parent = Some(curr.clone());
                    if key < curr_val {
                        let mut left = curr.borrow().left.clone();
                        DELETE(&mut left, key, parent, true)
                    } else {
                        let mut right = curr.borrow().right.clone();
                        DELETE(&mut right, key, parent, false)
                    }
                }
            }
        };

        DELETE(&mut root, key, None, false);

        root
    }

    fn iter_helper(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut parent: Option<Rc<RefCell<TreeNode>>> = None;
        let mut is_left = false;

        let mut curr_node = root.clone();
        while let Some(curr) = curr_node {
            let curr_val = curr.borrow().val;

            if curr_val == key {
                let left = curr.borrow_mut().left.take();
                let right = curr.borrow_mut().right.take();

                let child = if right.is_some() {
                    let mut leftmost = right.clone();
                    while let Some(ref curr) = leftmost {
                        let left = curr.borrow().left.clone();
                        if left.is_none() {
                            break;
                        }
                        leftmost = left;
                    }

                    if let Some(curr) = leftmost {
                        curr.borrow_mut().left = left;
                    }

                    right
                } else {
                    left
                };

                if let Some(p) = parent {
                    if is_left {
                        p.borrow_mut().left = child;
                    } else {
                        p.borrow_mut().right = child;
                    }
                } else {
                    root = child;
                }

                break;
            } else {
                parent = Some(curr.clone());
                if key < curr_val {
                    is_left = true;
                    curr_node = curr.borrow().left.clone();
                } else {
                    is_left = false;
                    curr_node = curr.borrow().right.clone();
                }
            }
        }

        root
    }
}
//leetcode submit region end(Prohibit modification and deletion)
