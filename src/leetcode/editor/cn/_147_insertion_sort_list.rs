//Given the head of a singly linked list, sort the list using insertion sort,
//and return the sorted list's head.
//
// The steps of the insertion sort algorithm:
//
//
// Insertion sort iterates, consuming one input element each repetition and
//growing a sorted output list.
// At each iteration, insertion sort removes one element from the input data,
//finds the location it belongs within the sorted list and inserts it there.
// It repeats until no input elements remain.
//
//
// The following is a graphical example of the insertion sort algorithm. The
//partially sorted list (black) initially contains only the first element in the
//list. One element (red) is removed from the input data and inserted in-place into
//the sorted list with each iteration.
//
//
// Example 1:
//
//
//Input: head = [4,2,1,3]
//Output: [1,2,3,4]
//
//
// Example 2:
//
//
//Input: head = [-1,5,3,4,0]
//Output: [-1,0,3,4,5]
//
//
//
// Constraints:
//
//
// The number of nodes in the list is in the range [1, 5000].
// -5000 <= Node.val <= 5000
//
//
// Related Topics Linked List Sorting 👍 2862 👎 844

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
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::iteration_helper(head)
        //Self::recursion_helper(head, None)
    }

    fn recursion_helper(
        head: Option<Box<ListNode>>,
        sort_head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (head, sort_head) {
            (None, None) => None,
            (None, Some(sort_head)) => Some(sort_head),
            (Some(mut head), None) => {
                let next_head = head.next.take();
                let sort_head = Some(head);
                Self::recursion_helper(next_head, sort_head)
            }
            (Some(mut head), Some(sort_head)) => {
                let next_head = head.next.take();
                let sort_head = Self::insert_sort(head, sort_head);
                Self::recursion_helper(next_head, Some(sort_head))
            }
        }
    }

    fn insert_sort(mut node: Box<ListNode>, mut sort_head: Box<ListNode>) -> Box<ListNode> {
        if node.val <= sort_head.val {
            node.next = Some(sort_head);
            return node;
        }

        let mut cur = sort_head.as_mut();
        while cur.next.is_some() {
            if cur.next.as_ref().unwrap().val >= node.val {
                break;
            }
            cur = cur.next.as_deref_mut().unwrap();
        }

        let next = cur.next.take();
        node.next = next;
        cur.next = Some(node);

        sort_head
    }

    fn iteration_helper(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(i32::MIN);

        while let Some(mut curr) = head {
            head = curr.next.take();

            let mut p = &mut dummy;
            while let Some(ref next) = p.next {
                if curr.val > next.val {
                    p = p.next.as_mut().unwrap();
                } else {
                    break;
                }
            }
            curr.next = p.next.take();
            p.next = Some(curr);
        }

        dummy.next
    }
}
//leetcode submit region end(Prohibit modification and deletion)
