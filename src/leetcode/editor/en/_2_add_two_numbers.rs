//You are given two non-empty linked lists representing two non-negative
//integers. The digits are stored in reverse order, and each of their nodes contains a
//single digit. Add the two numbers and return the sum as a linked list.
//
// You may assume the two numbers do not contain any leading zero, except the
//number 0 itself.
//
//
// Example 1:
//
//
//Input: l1 = [2,4,3], l2 = [5,6,4]
//Output: [7,0,8]
//Explanation: 342 + 465 = 807.
//
//
// Example 2:
//
//
//Input: l1 = [0], l2 = [0]
//Output: [0]
//
//
// Example 3:
//
//
//Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
//Output: [8,9,9,9,0,0,0,1]
//
//
//
// Constraints:
//
//
// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have
//leading zeros.
//
//
// Related Topics Linked List Math Recursion 👍 26186 👎 5072
#![allow(dead_code)]
#![allow(unused)]

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        if l1.is_none() {
            return l2;
        }
        if l2.is_none() {
            return l1;
        }

        let mut size = 0; // 生成的链表中节点个数
        let mut result = None;
        let mut l1_node = l1.as_ref().unwrap();
        let mut l2_node = l2.as_ref().unwrap();
        let mut greater_than_nine = false;
        loop {
            let l1_val = l1_node.val;
            let l2_val = l2_node.val;

            let mut sum = l1_val + l2_val;
            if greater_than_nine {
                greater_than_nine = false;
                sum += 1;
            }
            if sum > 9 {
                greater_than_nine = true;
                sum -= 10;
            }

            let new_node = Some(Box::new(ListNode::new(sum)));
            if result.is_none() {
                result = new_node;
            } else {
                let mut head = result.as_mut().unwrap();
                /* while let Some(node) = head.next.as_mut() {
                    head = node;
                }*/
                for _ in 0..size - 1 {
                    head = head.next.as_mut().unwrap();
                }
                head.next = new_node;
            }
            size += 1;

            if l1_node.next.is_none() || l2_node.next.is_none() {
                break;
            }
            l1_node = l1_node.next.as_ref().unwrap();
            l2_node = l2_node.next.as_ref().unwrap();
        }

        while let Some(node) = l1_node.next.as_ref() {
            let mut val = node.val;
            if greater_than_nine {
                greater_than_nine = false;
                val += 1;
            }
            if val > 9 {
                greater_than_nine = true;
                val -= 10;
            }
            let new = Some(Box::new(ListNode::new(val)));

            let mut head = result.as_mut().unwrap();
            /*while let Some(node) = head.next.as_mut() {
                head = node;
            }*/
            for _ in 0..size - 1 {
                head = head.next.as_mut().unwrap();
            }

            head.next = new;
            size += 1;
            l1_node = node;
        }

        while let Some(node) = l2_node.next.as_ref() {
            let mut val = node.val;
            if greater_than_nine {
                greater_than_nine = false;
                val += 1;
            }
            if val > 9 {
                greater_than_nine = true;
                val -= 10;
            }
            let new = Some(Box::new(ListNode::new(val)));

            let mut head = result.as_mut().unwrap();
            /* while let Some(node) = head.next.as_mut() {
                head = node;
            }*/
            for _ in 0..size - 1 {
                head = head.next.as_mut().unwrap();
            }

            head.next = new;
            size += 1;
            l2_node = node;
        }

        if greater_than_nine {
            greater_than_nine = false;
            let new = Some(Box::new(ListNode::new(1)));

            let mut head = result.as_mut().unwrap();
            /*while let Some(node) = head.next.as_mut() {
                head = node;
            }*/
            for _ in 0..size - 1 {
                head = head.next.as_mut().unwrap();
            }

            head.next = new;
            size += 1;
        }

        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)
