//You are given the head of a singly linked-list. The list can be represented
//as:
//
//
//L0 → L1 → … → Ln - 1 → Ln
//
//
// Reorder the list to be on the following form:
//
//
//L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
//
//
// You may not modify the values in the list's nodes. Only nodes themselves may
//be changed.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4]
//Output: [1,4,2,3]
//
//
// Example 2:
//
//
//Input: head = [1,2,3,4,5]
//Output: [1,5,2,4,3]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [1, 5 * 10⁴].
// 1 <= Node.val <= 1000
//
//
// Related Topics Linked List Two Pointers Stack Recursion 👍 9529 👎 314

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

    pub fn new2(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode { val, next }))
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let res = Self::rebuild_list(head.take());
        head.replace(res);
    }

    pub fn rebuild_list(mut head: Option<Box<ListNode>>) -> Box<ListNode> {
        let mut nodes = vec![];
        while let Some(mut curr) = head {
            head = curr.next.take();
            nodes.push(curr);
        }

        let mut start = 0;
        let mut end = nodes.len() - 1;
        let mut stack = vec![];
        while start < end {
            stack.push(nodes[start].clone());
            stack.push(nodes[end].clone());

            start += 1;
            end -= 1;
        }
        if start == end {
            stack.push(nodes[start].clone());
        }

        let mut new_haed = None;
        while let Some(mut curr) = stack.pop() {
            if new_haed.is_none() {
                new_haed = Some(curr);
            } else {
                curr.next = new_haed;
                new_haed = Some(curr);
            }
        }

        new_haed.unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
