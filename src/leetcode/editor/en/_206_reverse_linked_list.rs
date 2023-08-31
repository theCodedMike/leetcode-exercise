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
        Self::iteration_helper(head)
        //Self::recursion_helper1(head)
        //Self::recursion_helper2(head).0
        //Self::recursion_helper3(head).0
    }

    pub fn iteration_helper(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head = None;

        while let Some(mut curr) = head {
            head = curr.next.take();

            if new_head.is_none() {
                new_head = Some(curr);
            } else {
                curr.next = new_head;
                new_head = Some(curr);
            }
        }

        new_head
    }

    pub fn recursion_helper1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut curr) => {
                let mut new_head = Self::recursion_helper1(curr.next.take());
                if new_head.is_none() {
                    Some(curr)
                } else {
                    let mut p = new_head.as_mut();
                    while let Some(p_curr) = p {
                        if p_curr.next.is_none() {
                            p_curr.next = Some(curr);
                            break;
                        }
                        p = p_curr.next.as_mut();
                    }

                    new_head
                }
            }
        }
    }

    pub fn recursion_helper2(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, usize) {
        match head {
            None => (None, 0),
            Some(mut curr) => {
                let (new_head, level) = Self::recursion_helper2(curr.next.take());
                match new_head {
                    None => (Some(curr), level + 1),
                    Some(mut new_head) => {
                        let mut p = &mut new_head;
                        for _ in 1..level {
                            p = p.next.as_mut().unwrap();
                        }
                        p.next = Some(curr);
                        (Some(new_head), level + 1)
                    }
                }
            }
        }
    }

    /*
    /// 编译无法通过
    pub fn recursion_helper3(
        head: Option<Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, Option<&'static mut Box<ListNode>>) {
        match head {
            None => (None, None),
            Some(mut curr) => {
                let (new_head, new_tail) = Self::recursion_helper3(curr.next.take());
                match new_head {
                    None => (Some(curr), None),
                    Some(mut new_head) => match new_tail {
                        None => {
                            let mut new_tail_node = Some(curr);
                            let p = new_tail_node.as_mut();
                            new_head.next = new_tail_node;
                            (Some(new_head), p)
                        }
                        Some(new_tail) => {
                            let mut new_tail_node = Some(curr);
                            let p = new_tail_node.as_mut();
                            new_tail.next = new_tail_node;
                            (Some(new_head), p)
                        }
                    },
                }
            }
        }
    }*/
}
//leetcode submit region end(Prohibit modification and deletion)
