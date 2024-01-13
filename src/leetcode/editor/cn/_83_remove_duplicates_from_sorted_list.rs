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
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut spider = head.as_mut();

        while let Some(curr) = spider.take() {
            let curr_val = curr.val;

            if curr.next.is_none() {
                break;
            }

            curr.next.take().map(|next| {
                let next_val = next.val;

                if curr_val == next_val {
                    curr.next = next.next;
                    spider = Some(curr);
                } else {
                    curr.next = Some(next);
                    spider = curr.next.as_mut();
                }
            });
        }

        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)
