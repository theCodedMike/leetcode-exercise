//Given the head of a singly linked list and two integers left and right where
//left <= right, reverse the nodes of the list from position left to position
//right, and return the reversed list.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], left = 2, right = 4
//Output: [1,4,3,2,5]
//
//
// Example 2:
//
//
//Input: head = [5], left = 1, right = 1
//Output: [5]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is n.
// 1 <= n <= 500
// -500 <= Node.val <= 500
// 1 <= left <= right <= n
//
//
//
//Follow up: Could you do it in one pass?
//
// Related Topics Linked List 👍 9603 👎 436

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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn new2(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn reverse_between(
        mut head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if left == right {
            return head;
        }

        let mut in_range = ListNode::new(0);
        let mut out_range = ListNode::new(0);
        let mut p_out_range = &mut out_range;
        let mut count = 0;
        let mut in_position_count = 0;

        while let Some(curr) = head {
            count += 1;

            if Solution::in_position(count, left, right) {
                in_position_count += 1;
                let mut new_node = ListNode::new(curr.val);
                new_node.next = in_range.next;
                in_range.next = Some(Box::new(new_node));
            } else {
                if in_position_count != 0 {
                    p_out_range.next = in_range.next.take();
                    for _ in 0..in_position_count {
                        p_out_range = p_out_range.next.as_mut().unwrap();
                    }
                    in_position_count = 0;
                }
                p_out_range.next = Some(Box::new(ListNode::new(curr.val)));
                p_out_range = p_out_range.next.as_mut().unwrap();
            }

            head = curr.next;

            if head.is_none() && in_position_count != 0 {
                // at end
                p_out_range.next = in_range.next.take();
            }
        }

        out_range.next
    }

    fn in_position(count: i32, left: i32, right: i32) -> bool {
        left <= count && count <= right
    }
}
//leetcode submit region end(Prohibit modification and deletion)
