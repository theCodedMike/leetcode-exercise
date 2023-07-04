//Given the head of a linked list, reverse the nodes of the list k at a time,
//and return the modified list.
//
// k is a positive integer and is less than or equal to the length of the
//linked list. If the number of nodes is not a multiple of k then left-out nodes, in
//the end, should remain as it is.
//
// You may not alter the values in the list's nodes, only nodes themselves may
//be changed.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], k = 2
//Output: [2,1,4,3,5]
//
//
// Example 2:
//
//
//Input: head = [1,2,3,4,5], k = 3
//Output: [3,2,1,4,5]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is n.
// 1 <= k <= n <= 5000
// 0 <= Node.val <= 1000
//
//
//
// Follow-up: Can you solve the problem in O(1) extra memory space?
//
// Related Topics Linked List Recursion 👍 11624 👎 590

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, _k: i32) -> Option<Box<ListNode>> {
        todo!();
        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)
