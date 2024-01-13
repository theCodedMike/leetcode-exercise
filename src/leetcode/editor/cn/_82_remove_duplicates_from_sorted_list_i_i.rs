//Given the head of a sorted linked list, delete all nodes that have duplicate
//numbers, leaving only distinct numbers from the original list. Return the linked
//list sorted as well.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,3,4,4,5]
//Output: [1,2,5]
//
//
// Example 2:
//
//
//Input: head = [1,1,1,2,3]
//Output: [2,3]
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
// Related Topics Linked List Two Pointers 👍 8016 👎 210

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
        ListNode { val, next: None }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;

        let mut duplicate = false;
        while let Some(curr) = head {
            if let Some(next) = curr.next.as_ref() {
                if curr.val == next.val {
                    duplicate = true;
                } else {
                    if !duplicate {
                        p.next = Some(Box::new(ListNode::new(curr.val)));
                        p = p.next.as_mut().unwrap();
                    }
                    duplicate = false;
                }
            } else {
                // last element
                if !duplicate {
                    p.next = Some(Box::new(ListNode::new(curr.val)));
                }
            }
            head = curr.next;
        }

        dummy.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
