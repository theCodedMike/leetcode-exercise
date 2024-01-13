use leetcode_exercise::leetcode::editor::cn::_160_intersection_of_two_linked_lists::{
    ListNode, Solution,
};
use std::ptr::null_mut;

#[test]
fn intersection_of_two_linked_lists_1() {
    let node_8 = ListNode::new(8, ListNode::new(4, ListNode::new(5, null_mut())));
    let head_a = ListNode::new(4, ListNode::new(1, node_8.clone()));
    let head_b = ListNode::new(5, ListNode::new(6, ListNode::new(1, node_8)));
    let res = Solution::get_intersection_node(head_a, head_b);
    assert_eq!(unsafe { (*res).val }, 8);
}

#[test]
fn intersection_of_two_linked_lists_2() {
    let node_2 = ListNode::new(2, ListNode::new(4, null_mut()));
    let head_a = ListNode::new(1, ListNode::new(9, ListNode::new(1, node_2.clone())));
    let head_b = ListNode::new(3, node_2);
    let res = Solution::get_intersection_node(head_a, head_b);
    assert_eq!(unsafe { (*res).val }, 2);
}

#[test]
fn intersection_of_two_linked_lists_3() {
    let head_a = ListNode::new(2, ListNode::new(6, ListNode::new(4, null_mut())));
    let head_b = ListNode::new(1, ListNode::new(5, null_mut()));
    let res = Solution::get_intersection_node(head_a, head_b);
    assert_eq!(res, null_mut());
}
