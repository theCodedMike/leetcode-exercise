//Given the head of a linked list, remove the nᵗʰ node from the end of the list
//and return its head.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], n = 2
//Output: [1,2,3,5]
//
//
// Example 2:
//
//
//Input: head = [1], n = 1
//Output: []
//
//
// Example 3:
//
//
//Input: head = [1,2], n = 1
//Output: [1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is sz.
// 1 <= sz <= 30
// 0 <= Node.val <= 100
// 1 <= n <= sz
//
//
//
// Follow up: Could you do this in one pass?
//
// Related Topics Linked List Two Pointers 👍 16136 👎 674

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        //Self::calc_length_of_listed_list(head, n)
        //Self::use_stack(head, n)
        Self::two_pointers(head, n)
        //Self::recursion_helper(head, n).1
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(1)
    fn calc_length_of_listed_list(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // calculate the len of linked list
        let mut counter = head.as_ref();
        let mut len = 0;
        while let Some(curr) = counter {
            len += 1;
            counter = curr.next.as_ref();
        }

        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut p = &mut dummy;
        // move p to the previous node of to be deleted node
        for _ in 0..(len - n) {
            p = p.next.as_mut().unwrap();
        }
        if let Some(mut to_be_del) = p.next.take() {
            p.next = to_be_del.next.take();
        }

        dummy.next
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn use_stack(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut stack = vec![];
        while let Some(mut curr) = head {
            head = curr.next.take();
            stack.push(curr);
        }

        let mut i = 0;
        let mut head = None;
        while let Some(mut curr) = stack.pop() {
            i += 1;
            if i != n {
                curr.next = head;
                head = Some(curr);
            }
        }

        head
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(1)
    fn two_pointers(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(-1);
        dummy.next = head;
        let mut fast = &dummy as *const ListNode;
        let mut slow = &mut dummy as *mut ListNode;

        unsafe {
            for _ in 0..n {
                fast = (*fast).next.as_deref().unwrap();
            }
            loop {
                if (*fast).next.is_none() {
                    break;
                }
                fast = (*fast).next.as_deref().unwrap();
                slow = (*slow).next.as_deref_mut().unwrap();
            }
            if let Some(mut to_be_del) = (*slow).next.take() {
                (*slow).next = to_be_del.next.take();
            }
        }

        dummy.next
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn recursion_helper(head: Option<Box<ListNode>>, n: i32) -> (i32, Option<Box<ListNode>>) {
        match head {
            None => (0, None),
            Some(mut curr) => {
                let (level, next) = Self::recursion_helper(curr.next.take(), n);
                curr.next = next;

                if level + 1 == n {
                    (level + 1, curr.next)
                } else {
                    (level + 1, Some(curr))
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
