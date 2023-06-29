//Given the head of a linked list, remove the nᵗʰ node from the end of the list
//and return its head.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], n = 2
//Output: [1,2,3,5]
//
//
// Example 2:
//
//
//Input: head = [1], n = 1
//Output: []
//
//
// Example 3:
//
//
//Input: head = [1,2], n = 1
//Output: [1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is sz.
// 1 <= sz <= 30
// 0 <= Node.val <= 100
// 1 <= n <= sz
//
//
//
// Follow up: Could you do this in one pass?
//
// Related Topics Linked List Two Pointers 👍 16136 👎 674

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // count of nodes
        let mut count = 0;
        let mut counter = head.as_ref();
        while let Some(node) = counter {
            count += 1;
            counter = node.next.as_ref();
        }
        // move the pointer to node which before the deleted node
        let mut new_head = ListNode::new(0);
        new_head.next = head;
        let mut helper = &mut new_head;
        for _ in 0..(count - n) {
            helper = helper.next.as_mut().unwrap();
        }
        // remove
        helper.next.take().map(|target| {
            helper.next = target.next;
        });

        new_head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
