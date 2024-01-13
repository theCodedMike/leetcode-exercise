//Given the head of a linked list, rotate the list to the right by k places.
//
//
// Example 1:
//
//
//Input: head = [1,2,3,4,5], k = 2
//Output: [4,5,1,2,3]
//
//
// Example 2:
//
//
//Input: head = [0,1,2], k = 4
//Output: [2,0,1]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [0, 500].
// -100 <= Node.val <= 100
// 0 <= k <= 2 * 10⁹
//
//
// Related Topics Linked List Two Pointers 👍 8318 👎 1393

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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut count = 0;
        let mut temp_head = head.as_ref();
        while let Some(node) = temp_head {
            count += 1;
            temp_head = node.next.as_ref();
        }
        if count == 0 {
            return head;
        }

        let real_k = k % count;
        if real_k == 0 {
            return head;
        }

        // 先将1 -> 2 -> 3放在另一个dummy list中，此时head指向节点4
        let mut dummy = ListNode::new(0);
        let mut p = &mut dummy;
        for _ in 1..=(count - real_k) {
            head.take().map(|node| {
                p.next = Some(Box::new(ListNode::new(node.val)));
                head = node.next;
            });
            p = p.next.as_mut().unwrap();
        }

        // 重新创建一个可变指针，移动到节点5(末尾)，将dummy list中保存的节点链接到末尾
        let mut temp_head = head.as_mut();
        for _ in 1..real_k {
            temp_head = temp_head.unwrap().next.as_mut();
        }
        temp_head.map(|node| {
            node.next = dummy.next;
        });

        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)
