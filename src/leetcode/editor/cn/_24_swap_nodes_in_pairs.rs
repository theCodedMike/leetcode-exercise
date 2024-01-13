//Given a linked list, swap every two adjacent nodes and return its head. You
//must solve the problem without modifying the values in the list's nodes (i.e.,
//only nodes themselves may be changed.)
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4]
//Output: [2,1,4,3]
//
//
// Example 2:
//
//
//Input: head = []
//Output: []
//
//
// Example 3:
//
//
//Input: head = [1]
//Output: [1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 100].
// 0 <= Node.val <= 100
//
//
// Related Topics Linked List Recursion 👍 10704 👎 388

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
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //Self::iteration_helper(head)
        Self::recursion_helper(head)
    }

    fn iteration_helper(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut p = &mut dummy;

        while let Some(mut curr) = p.next.take() {
            match curr.next.take() {
                None => {
                    p.next = Some(curr);
                    break;
                }
                Some(mut next) => {
                    curr.next = next.next.take();
                    next.next = Some(curr);
                    p.next = Some(next);
                    p = p.next.as_mut().unwrap().next.as_mut().unwrap();
                }
            }
        }

        dummy.next
    }

    fn iteration_helper2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut p = &mut head;

        while let Some(mut curr) = p.take() {
            match curr.next.take() {
                None => {
                    *p = Some(curr);
                    break;
                }
                Some(mut next) => {
                    curr.next = next.next.take();
                    next.next = Some(curr);
                    *p = Some(next);
                    p = &mut p.as_mut().unwrap().next.as_mut().unwrap().next;
                }
            }
        }

        head
    }

    fn recursion_helper(curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match curr {
            None => None,
            Some(mut curr) => match curr.next.take() {
                None => Some(curr),
                Some(mut next) => {
                    curr.next = Self::recursion_helper(next.next.take());
                    next.next = Some(curr);
                    Some(next)
                }
            },
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
