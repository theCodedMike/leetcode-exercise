//You are given the root node of a binary search tree (BST) and a value to
//insert into the tree. Return the root node of the BST after the insertion. It is
//guaranteed that the new value does not exist in the original BST.
//
// Notice that there may exist multiple valid ways for the insertion, as long
//as the tree remains a BST after insertion. You can return any of them.
//
//
// Example 1:
//
//
//Input: root = [4,2,7,1,3], val = 5
//Output: [4,2,7,1,3,5]
//Explanation: Another accepted tree is:
//
//
//
// Example 2:
//
//
//Input: root = [40,20,60,10,30,50,70], val = 25
//Output: [40,20,60,10,30,50,70,null,null,25]
//
//
// Example 3:
//
//
//Input: root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
//Output: [4,2,7,1,3,5]
//
//
//
// Constraints:
//
//
// The number of nodes in the tree will be in the range [0, 10⁴].
// -10⁸ <= Node.val <= 10⁸
// All the values Node.val are unique.
// -10⁸ <= val <= 10⁸
// It's guaranteed that val does not exist in the original BST.
//
//
// Related Topics 树 二叉搜索树 二叉树 👍 546 👎 0

#![allow(dead_code)]

pub struct Solution;
use crate::binary_tree::safe::TreeNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::iter_helper_1(root, val)
        //Self::iter_helper_2(root, val)

        //Self::recur_helper_1(root, val)
        Self::recur_helper_2(root, val)
    }

    fn iter_helper_1(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let new = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        if root.is_none() {
            return new;
        }

        let mut root_node = root.clone();

        while let Some(curr) = root_node {
            let curr_val = curr.borrow().val;

            let (next, is_right) = if val > curr_val {
                (curr.borrow().right.clone(), true)
            } else {
                (curr.borrow().left.clone(), false)
            };

            if next.is_some() {
                root_node = next;
            } else {
                if is_right {
                    curr.borrow_mut().right = new;
                } else {
                    curr.borrow_mut().left = new;
                }
                break;
            }
        }

        root
    }

    fn iter_helper_2(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let new = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        if root.is_none() {
            return new;
        }

        let mut root_node = root.clone();

        while let Some(curr) = root_node {
            let curr_val = curr.borrow().val;

            if val > curr_val {
                let right = curr.borrow().right.clone();
                if right.is_some() {
                    root_node = right;
                } else {
                    curr.borrow_mut().right = new;
                    break;
                }
            } else {
                let left = curr.borrow().left.clone();
                if left.is_some() {
                    root_node = left;
                } else {
                    curr.borrow_mut().left = new;
                    break;
                }
            }
        }

        root
    }

    fn recur_helper_1(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let new = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        if root.is_none() {
            return new;
        }

        const TRAVERSAL: fn(Option<Rc<RefCell<TreeNode>>>, i32, Option<Rc<RefCell<TreeNode>>>) =
            |root, val, new| {
                if let Some(curr) = root {
                    let curr_val = curr.borrow().val;

                    if val > curr_val {
                        let right = curr.borrow().right.clone();
                        if right.is_some() {
                            TRAVERSAL(right, val, new);
                        } else {
                            curr.borrow_mut().right = new;
                        }
                    } else {
                        let left = curr.borrow().left.clone();
                        if left.is_some() {
                            TRAVERSAL(left, val, new);
                        } else {
                            curr.borrow_mut().left = new;
                        }
                    }
                }
            };
        TRAVERSAL(root.clone(), val, new);

        root
    }

    fn recur_helper_2(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        const TRAVERSAL: fn(Option<Rc<RefCell<TreeNode>>>, i32) -> Option<Rc<RefCell<TreeNode>>> =
            |root, val| match root {
                None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
                Some(curr) => {
                    let curr_val = curr.borrow().val;

                    if val > curr_val {
                        let right = curr.borrow_mut().right.take();
                        curr.borrow_mut().right = TRAVERSAL(right, val);
                    } else {
                        let left = curr.borrow_mut().left.take();
                        curr.borrow_mut().left = TRAVERSAL(left, val);
                    }

                    Some(curr)
                }
            };

        TRAVERSAL(root, val)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
