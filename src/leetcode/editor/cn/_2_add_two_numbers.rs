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
        ListNode { val, next: None }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = Box::new(ListNode::new(0)); // 结果链表的头节点
        let mut spider = &mut head; // 结果链表的最后一个节点

        while l1.is_some() || l2.is_some() || carry == 1 {
            let sum = match l1 {
                None => 0,
                Some(n1) => {
                    l1 = n1.next;
                    n1.val
                }
            } + match l2 {
                None => 0,
                Some(n2) => {
                    l2 = n2.next;
                    n2.val
                }
            } + carry;

            carry = sum / 10;

            spider.next = Some(Box::new(ListNode::new(sum % 10)));
            spider = spider.next.as_mut().unwrap();
        }

        head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
