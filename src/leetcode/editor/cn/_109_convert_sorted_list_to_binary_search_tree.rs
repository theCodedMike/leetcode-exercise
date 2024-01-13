//Given the head of a singly linked list where elements are sorted in ascending
//order, convert it to a height-balanced binary search tree.
//
//
// Example 1:
//
//
//Input: head = [-10,-3,0,5,9]
//Output: [0,-3,9,-10,null,5]
//Explanation: One possible answer is [0,-3,9,-10,null,5], which represents the
//shown height balanced BST.
//
//
// Example 2:
//
//
//Input: head = []
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in head is in the range [0, 2 * 10⁴].
// -10⁵ <= Node.val <= 10⁵
//
//
// Related Topics Linked List Divide and Conquer Tree Binary Search Tree Binary
//Tree 👍 7081 👎 149

#![allow(dead_code)]

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn new2(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }
}
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
use std::ops::Index;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::convert_list_to_vec_then_recursively_build(head)
        //Self::calc_len_then_recursively_build(head)
        Self::slow_and_fast_ptr(head.as_ref(), None)
    }

    /// Solution 1
    ///
    /// 1、list -> vec
    ///
    /// 2、vec -> bst
    fn convert_list_to_vec_then_recursively_build(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nums = vec![];
        while let Some(curr) = head {
            nums.push(curr.val);
            head = curr.next;
        }

        const RECURSION_HELPER: fn(&[i32]) -> Option<Rc<RefCell<TreeNode>>> =
            |nodes: &[i32]| -> Option<Rc<RefCell<TreeNode>>> {
                let len = nodes.len();
                if len == 0 {
                    return None;
                }

                let root_idx = len / 2;
                let root = Rc::new(RefCell::new(TreeNode::new(nodes[root_idx])));
                if len == 1 {
                    return Some(root);
                }

                root.borrow_mut().left = RECURSION_HELPER(nodes.index(..root_idx));
                root.borrow_mut().right = RECURSION_HELPER(nodes.index((root_idx + 1)..));

                Some(root)
            };

        RECURSION_HELPER(&nums)
    }

    /// Solution 2
    ///
    /// 1、calc len
    ///
    /// 2、build tree recursively
    fn calc_len_then_recursively_build(
        head: Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let calc_len = |mut head: Option<&Box<ListNode>>| -> i32 {
            let mut len = 0;
            while let Some(curr) = head {
                len += 1;
                head = curr.next.as_ref()
            }
            len
        };

        let len = calc_len(head.as_ref());
        let (_, tree) = Self::build_tree(head.as_ref(), 0, len);
        tree
    }

    fn build_tree(
        head: Option<&Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> (Option<&Box<ListNode>>, Option<Rc<RefCell<TreeNode>>>) {
        if left >= right {
            return (head, None);
        }

        let mid = (left + right) / 2;
        let (head, left) = Self::build_tree(head, left, mid);
        if let Some(head) = head {
            let mut root = TreeNode::new(head.val);
            root.left = left;
            let (head, right) = Self::build_tree(head.next.as_ref(), mid + 1, right);
            root.right = right;
            return (head, Some(Rc::new(RefCell::new(root))));
        }

        (head, None)
    }

    /// Solution 3: double ptr
    fn slow_and_fast_ptr(
        left: Option<&Box<ListNode>>,
        right: Option<&Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if Self::are_ptrs_eq(left, right) {
            return None;
        }
        let mid = Self::get_mid(left, right);
        if let Some(mid_node) = mid {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: mid_node.val,
                left: Self::slow_and_fast_ptr(left, mid),
                right: Self::slow_and_fast_ptr(Self::next_node(mid), right),
            })));
        }
        None
    }

    fn get_mid<'a>(
        left: Option<&'a Box<ListNode>>,
        right: Option<&'a Box<ListNode>>,
    ) -> Option<&'a Box<ListNode>> {
        let mut fast = left;
        let mut slow = left;

        // while fast != right && fast.next != right {
        //     fast = fast.next.next;
        //     slow = slow.next;
        // }
        while !Self::are_ptrs_eq(fast, right) && !Self::are_ptrs_eq(Self::next_node(fast), right) {
            fast = Self::next_node(Self::next_node(fast));
            slow = Self::next_node(slow);
        }

        slow
    }

    fn next_node(n: Option<&Box<ListNode>>) -> Option<&Box<ListNode>> {
        n.and_then(|n| n.next.as_ref())
    }

    fn are_ptrs_eq(p1: Option<&Box<ListNode>>, p2: Option<&Box<ListNode>>) -> bool {
        match (p1, p2) {
            (None, None) => true,
            (Some(p1), Some(p2)) => std::ptr::eq(p1, p2),
            _ => false,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
