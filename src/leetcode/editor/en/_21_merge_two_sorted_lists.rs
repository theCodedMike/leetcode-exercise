//You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists in a one sorted list. The list should be made by
//splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
// Example 1:
//
//
//Input: list1 = [1,2,4], list2 = [1,3,4]
//Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
//
//Input: list1 = [], list2 = []
//Output: []
//
//
// Example 3:
//
//
//Input: list1 = [], list2 = [0]
//Output: [0]
//
//
//
// Constraints:
//
//
// The number of nodes in both lists is in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.
//
//
// Related Topics Linked List Recursion 👍 18571 👎 1727

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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;

        while list1.is_some() || list2.is_some() {
            match (list1.as_ref(), list2.as_ref()) {
                (None, None) => {
                    // 这种情况应该不会出现, 在while那里有判断
                }
                (Some(v1), None) => {
                    tail.next = Some(Box::new(ListNode::new(v1.val)));
                    tail = tail.next.as_mut().unwrap();
                    list1 = list1.unwrap().next;
                }
                (None, Some(v2)) => {
                    tail.next = Some(Box::new(ListNode::new(v2.val)));
                    tail = tail.next.as_mut().unwrap();
                    list2 = list2.unwrap().next;
                }
                (Some(v1), Some(v2)) => {
                    if v1.val <= v2.val {
                        tail.next = Some(Box::new(ListNode::new(v1.val)));
                        tail = tail.next.as_mut().unwrap();
                        list1 = list1.unwrap().next;
                    } else {
                        tail.next = Some(Box::new(ListNode::new(v2.val)));
                        tail = tail.next.as_mut().unwrap();
                        list2 = list2.unwrap().next;
                    }
                }
            }
        }

        head.next
    }
}

//leetcode submit region end(Prohibit modification and deletion)
