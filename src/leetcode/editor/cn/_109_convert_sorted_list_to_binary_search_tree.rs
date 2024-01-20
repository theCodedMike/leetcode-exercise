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
use crate::binary_tree::safe::TreeNode;
use crate::linked_list::singly_linked_list::safe::ListNode;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        //Self::divide_and_conquer(head)

        Self::optimize_divide_and_conquer(head)
    }

    ///
    /// Time Complexity: O(nlog(n))
    /// Space Complexity: O(log(n))
    ///
    fn divide_and_conquer(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        const GET_MID: for<'a> fn(
            Option<&'a Box<ListNode>>,
            Option<&'a Box<ListNode>>,
        ) -> Option<&'a Box<ListNode>> = |left, right| {
            let mut slow = left;
            let mut fast = left;
            while fast != right && fast.is_some_and(|fast| fast.next.as_ref() != right) {
                fast.map(|f| {
                    fast = f.next.as_ref();
                });
                fast.map(|f| {
                    fast = f.next.as_ref();
                });
                slow.map(|s| {
                    slow = s.next.as_ref();
                });
            }

            slow
        };

        const CONVERT: fn(
            Option<&Box<ListNode>>,
            Option<&Box<ListNode>>,
        ) -> Option<Rc<RefCell<TreeNode>>> = |left, right| {
            if left == right {
                return None;
            }
            let mid = GET_MID(left, right);
            let mut root = if let Some(mid) = mid {
                Some(Rc::new(RefCell::new(TreeNode::new(mid.val))))
            } else {
                None
            };

            root.as_mut().map(|curr| {
                curr.borrow_mut().left = CONVERT(left, mid);
                curr.borrow_mut().right = CONVERT(mid.and_then(|mid| mid.next.as_ref()), right);
            });

            root
        };

        CONVERT(head.as_ref(), None)
    }

    ///
    /// Time Complexity: O(n)
    /// Space Complexity: O(log(n))
    ///
    fn optimize_divide_and_conquer(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let get_len = |mut curr: Option<&Box<ListNode>>| {
            let mut len = 0;
            while let Some(c) = curr {
                len += 1;
                curr = c.next.as_ref();
            }
            len
        };
        let len = get_len(head.as_ref());

        const CONVERT: fn(
            &mut Option<Box<ListNode>>,
            usize,
            usize,
        ) -> Option<Rc<RefCell<TreeNode>>> = |head, left, right| {
            if left == right {
                return None;
            }

            let mid = (left + right) / 2;
            let root = Rc::new(RefCell::new(TreeNode::new(0)));
            root.borrow_mut().left = CONVERT(head, left, mid);

            root.borrow_mut().val = head.take().map_or(0, |mut h| {
                let val = h.val;
                *head = h.next.take();
                val
            });

            root.borrow_mut().right = CONVERT(head, mid + 1, right);

            Some(root)
        };

        CONVERT(&mut head, 0, len)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
