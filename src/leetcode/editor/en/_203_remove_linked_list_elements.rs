//Given the head of a linked list and an integer val, remove all the nodes of
//the linked list that has Node.val == val, and return the new head.
//
//
// Example 1:
//
//
//Input: head = [1,2,6,3,4,5,6], val = 6
//Output: [1,2,3,4,5]
//
//
// Example 2:
//
//
//Input: head = [], val = 1
//Output: []
//
//
// Example 3:
//
//
//Input: head = [7,7,7,7], val = 7
//Output: []
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 10⁴].
// 1 <= Node.val <= 50
// 0 <= val <= 50
//
//
// Related Topics Linked List Recursion 👍 7862 👎 219

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

    pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = vec![];

        while let Some(mut curr) = head {
            res.push(curr.val);
            head = curr.next.take();
        }

        res
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        //Self::iteration_helper(head, val)
        Self::recursion_helper(head, val)
    }

    fn iteration_helper(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(head) => {
                let mut dummy = ListNode::new(-1);
                dummy.next = Some(head);
                let mut p = &mut dummy;

                while let Some(curr) = p.next.take() {
                    if curr.val == val {
                        p.next = curr.next;
                    } else {
                        p.next = Some(curr);
                        p = p.next.as_mut().unwrap();
                    }
                }

                dummy.next
            }
        }
    }

    fn recursion_helper(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut head) => {
                head.next = Self::recursion_helper(head.next.take(), val);
                if head.val == val {
                    head.next
                } else {
                    Some(head)
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
