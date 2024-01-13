//Given the head of a singly linked list, reverse the list, and return the
//reversed list.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5]
//Output: [5,4,3,2,1]
//
//
// Example 2:
//
//
//Input: head = [1,2]
//Output: [2,1]
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
// The number of nodes in the list is the range [0, 5000].
// -5000 <= Node.val <= 5000
//
//
//
// Follow up: A linked list can be reversed either iteratively or recursively.
//Could you implement both?
//
// Related Topics Linked List Recursion 👍 19346 👎 349

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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //Self::iteration_helper(head)
        Self::recursion_helper(None, head)
    }

    pub fn iteration_helper(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = new_head;
            new_head = Some(node);
        }

        new_head
    }

    pub fn recursion_helper(
        prev: Option<Box<ListNode>>,
        curr: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match curr {
            None => prev,
            Some(mut curr) => {
                let next = curr.next.take();
                curr.next = prev;
                Self::recursion_helper(Some(curr), next)
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
