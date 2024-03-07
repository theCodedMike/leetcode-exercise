use leetcode_rust::leetcode::editor::cn::_143_reorder_list::{ListNode, Solution};

#[test]
fn reorder_list() {
    let mut head = ListNode::new2(
        1,
        ListNode::new2(2, ListNode::new2(3, ListNode::new2(4, None))),
    );
    Solution::reorder_list(&mut head);
}
