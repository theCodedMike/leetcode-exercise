//Given the head of a sorted linked list, delete all duplicates such that each
//element appears only once. Return the linked list sorted as well.
//
//
// Example 1:
//
//
//Input: head = [1,1,2]
//Output: [1,2]
//
//
// Example 2:
//
//
//Input: head = [1,1,2,3,3]
//Output: [1,2,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 300].
// -100 <= Node.val <= 100
// The list is guaranteed to be sorted in ascending order.
//
//
// Related Topics Linked List 👍 7409 👎 250

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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut dummy = ListNode::new(i32::MIN);
        let mut tail = &mut dummy;

        while let Some(curr) = head {
            let val = curr.val;

            if tail.val != val {
                tail.next = Some(Box::new(ListNode::new(val)));
                tail = tail.next.as_mut().unwrap();
            }

            head = curr.next;
        }

        dummy.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
