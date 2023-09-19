//Implement the BSTIterator class that represents an iterator over the in-order
//traversal of a binary search tree (BST):
//
//
// BSTIterator(TreeNode root) Initializes an object of the BSTIterator class.
//The root of the BST is given as part of the constructor. The pointer should be
//initialized to a non-existent number smaller than any element in the BST.
// boolean hasNext() Returns true if there exists a number in the traversal to
//the right of the pointer, otherwise returns false.
// int next() Moves the pointer to the right, then returns the number at the
//pointer.
//
//
// Notice that by initializing the pointer to a non-existent smallest number,
//the first call to next() will return the smallest element in the BST.
//
// You may assume that next() calls will always be valid. That is, there will
//be at least a next number in the in-order traversal when next() is called.
//
//
// Example 1:
//
//
//Input
//["BSTIterator", "next", "next", "hasNext", "next", "hasNext", "next",
//"hasNext", "next", "hasNext"]
//[[[7, 3, 15, null, null, 9, 20]], [], [], [], [], [], [], [], [], []]
//Output
//[null, 3, 7, true, 9, true, 15, true, 20, false]
//
//
//Explanation
//BSTIterator bSTIterator = new BSTIterator([7, 3, 15, null, null, 9, 20]);
//bSTIterator.next(); // return 3
//bSTIterator.next(); // return 7
//bSTIterator.hasNext(); // return True
//bSTIterator.next(); // return 9
//bSTIterator.hasNext(); // return True
//bSTIterator.next(); // return 15
//bSTIterator.hasNext(); // return True
//bSTIterator.next(); // return 20
//bSTIterator.hasNext(); // return False
//
//
//
// Constraints:
//
//
// The number of nodes in the tree is in the range [1, 10⁵].
// 0 <= Node.val <= 10⁶
// At most 10⁵ calls will be made to hasNext, and next.
//
//
//
// Follow up:
//
//
// Could you implement next() and hasNext() to run in average O(1) time and use
//O(h) memory, where h is the height of the tree?
//
//
// Related Topics Stack Tree Design Binary Search Tree Binary Tree Iterator 👍 8
//066 👎 467

#![allow(dead_code)]

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
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
    pub fn new2(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;

pub struct BSTIterator {
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator {
            root: Self::convert(root),
        }
    }
    ///
    /// 中序遍历，将二叉树转换为链表
    ///
    fn convert(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = vec![];
        Self::recursive_traversal(&mut nodes, root);
        for i in 1..nodes.len() {
            nodes[i - 1].borrow_mut().right = Some(nodes[i].clone());
        }
        Some(nodes[0].clone())
    }
    fn recursive_traversal(
        nodes: &mut Vec<Rc<RefCell<TreeNode>>>,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) {
        match root {
            None => {}
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                Self::recursive_traversal(nodes, left);
                nodes.push(node);
                Self::recursive_traversal(nodes, right);
            }
        }
    }

    pub fn next(&mut self) -> i32 {
        self.root
            .take()
            .map(|root| {
                self.root = root.borrow_mut().right.take();
                root.borrow().val
            })
            .unwrap_or_default()
    }

    pub fn has_next(&self) -> bool {
        self.root.is_some()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
