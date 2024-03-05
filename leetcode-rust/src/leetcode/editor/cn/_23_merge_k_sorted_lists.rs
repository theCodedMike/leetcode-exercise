//You are given an array of k linked-lists lists, each linked-list is sorted in
//ascending order.
//
// Merge all the linked-lists into one sorted linked-list and return it.
//
//
// Example 1:
//
//
//Input: lists = [[1,4,5],[1,3,4],[2,6]]
//Output: [1,1,2,3,4,4,5,6]
//Explanation: The linked-lists are:
//[
//  1->4->5,
//  1->3->4,
//  2->6
//]
//merging them into one sorted list:
//1->1->2->3->4->4->5->6
//
//
// Example 2:
//
//
//Input: lists = []
//Output: []
//
//
// Example 3:
//
//
//Input: lists = [[]]
//Output: []
//
//
//
// Constraints:
//
//
// k == lists.length
// 0 <= k <= 10⁴
// 0 <= lists[i].length <= 500
// -10⁴ <= lists[i][j] <= 10⁴
// lists[i] is sorted in ascending order.
// The sum of lists[i].length will not exceed 10⁴.
//
//
// Related Topics Linked List Divide and Conquer Heap (Priority Queue) Merge
//Sort 👍 17412 👎 629

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
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { val, next }
    }
}

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut p = &mut head;
        for (idx, mut list) in lists.iter().enumerate() {
            if list.is_none() {
                continue;
            }

            while let Some(node) = list {
                if idx == 0 {
                    // 处理第一个非None list
                    p.next = Some(Box::new(ListNode::new(node.val)));
                    p = p.next.as_mut().unwrap();
                    list = &node.next;
                } else {
                    match p.next {
                        None => {
                            p.next = Some(Box::new(ListNode::new(node.val)));
                            list = &node.next;
                        }
                        Some(ref p_next) => {
                            let p_next_val = p_next.val;
                            if node.val < p_next_val {
                                let mut new = ListNode::new(node.val);
                                p.next.take().map(|node| {
                                    new.next = Some(node);
                                    p.next = Some(Box::new(new));
                                });
                                list = &node.next;
                            }
                            p = p.next.as_mut().unwrap();
                        }
                    }
                }
            }

            p = &mut head;
        }

        head.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
