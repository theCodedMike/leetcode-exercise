//Given the head of a linked list, return the list after sorting it in
//ascending order.
//
//
// Example 1:
//
//
//Input: head = [4,2,1,3]
//Output: [1,2,3,4]
//
//
// Example 2:
//
//
//Input: head = [-1,5,3,4,0]
//Output: [-1,0,3,4,5]
//
//
// Example 3:
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
// The number of nodes in the list is in the range [0, 5 * 10⁴].
// -10⁵ <= Node.val <= 10⁵
//
//
//
// Follow up: Can you sort the linked list in O(n logn) time and O(1) memory (i.
//e. constant space)?
//
// Related Topics Linked List Two Pointers Divide and Conquer Sorting Merge
//Sort 👍 10610 👎 303

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
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut len = 0;
        let mut ptr = &head;
        while let Some(node) = ptr {
            len += 1;
            ptr = &node.next;
        }
        let mut ptr = &mut head;
        for _ in 0..len / 2 {
            if let Some(ref mut node) = ptr {
                ptr = &mut node.next;
            }
        }
        let next = ptr.take();
        let l1 = Self::sort_list(head);
        let l2 = Self::sort_list(next);
        Self::merge(l1, l2)
    }

    fn merge(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, Some(n)) | (Some(n), None) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    n1.next = Self::merge(n1.next.take(), Some(n2));
                    Some(n1)
                } else {
                    n2.next = Self::merge(Some(n1), n2.next.take());
                    Some(n2)
                }
            }
            (None, None) => None,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
