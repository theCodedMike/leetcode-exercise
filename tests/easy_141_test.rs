use leetcode_exercise::leetcode::editor::en::_141_linked_list_cycle::{ListNode, Solution};
use std::ptr::null_mut;

#[test]
fn linked_list_cycle_1() {
    // 3 -> 2 -> 0 -> -4
    //      ↑__________|
    let node_2 = ListNode::new(2, null_mut());
    let node_0 = ListNode::new(0, ListNode::new(-4, node_2));
    unsafe {
        (*node_2).next = node_0;
    }
    let head = ListNode::new(3, node_2);
    assert_eq!(Solution::has_cycle(head), true);
}

#[test]
fn linked_list_cycle_2() {
    // 1 -> 2
    // ↑____|
    let head = ListNode::new(1, null_mut());
    let node_2 = ListNode::new(2, head);
    unsafe {
        (*head).next = node_2;
    }
    assert_eq!(Solution::has_cycle(head), true);
}

#[test]
fn linked_list_cycle_3() {
    // 1
    let head = ListNode::new(1, null_mut());
    assert_eq!(Solution::has_cycle(head), false);
}

#[test]
fn linked_list_cycle_4() {
    // 1 -> 1 -> 1 -> 1
    let head = ListNode::new(
        1,
        ListNode::new(1, ListNode::new(1, ListNode::new(1, null_mut()))),
    );
    assert_eq!(Solution::has_cycle(head), false);
}

#[test]
fn linked_list_cycle_5() {
    // empty
    let head = null_mut();
    assert_eq!(Solution::has_cycle(head), false);
}
