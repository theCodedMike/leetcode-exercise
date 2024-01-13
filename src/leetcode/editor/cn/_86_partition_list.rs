//Given the head of a linked list and a value x, partition it such that all
//nodes less than x come before nodes greater than or equal to x.
//
// You should preserve the original relative order of the nodes in each of the
//two partitions.
//
//
// Example 1:
//
//
//Input: head = [1,4,3,2,5,2], x = 3
//Output: [1,2,2,4,3,5]
//
//
// Example 2:
//
//
//Input: head = [2,1], x = 2
//Output: [1,2]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 200].
// -100 <= Node.val <= 100
// -200 <= x <= 200
//
//
// Related Topics Linked List Two Pointers 👍 5711 👎 657

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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt_x = ListNode::new(0);
        let mut p_of_lt_x = &mut lt_x;
        let mut ge_x = ListNode::new(0);
        let mut p_of_ge_x = &mut ge_x;

        while let Some(curr) = head {
            if curr.val < x {
                p_of_lt_x.next = Some(Box::new(ListNode::new(curr.val)));
                p_of_lt_x = p_of_lt_x.next.as_mut().unwrap();
            } else {
                p_of_ge_x.next = Some(Box::new(ListNode::new(curr.val)));
                p_of_ge_x = p_of_ge_x.next.as_mut().unwrap();
            }

            head = curr.next;
        }
        p_of_lt_x.next = ge_x.next;

        lt_x.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
