use leetcode_exercise::leetcode::editor::en::_203_remove_linked_list_elements::{
    ListNode, Solution,
};

#[test]
fn remove_linked_list_elements_1() {
    let head = ListNode::new2(
        1,
        ListNode::new2(
            2,
            ListNode::new2(
                6,
                ListNode::new2(
                    3,
                    ListNode::new2(4, ListNode::new2(5, ListNode::new2(6, None))),
                ),
            ),
        ),
    );
    let val = 6;
    let res = Solution::remove_elements(head, val);
    assert_eq!(ListNode::to_vec(res), [1, 2, 3, 4, 5]);
}

#[test]
fn remove_linked_list_elements_2() {
    let val = 1;
    let res = Solution::remove_elements(None, val);
    assert_eq!(ListNode::to_vec(res), []);
}

#[test]
fn remove_linked_list_elements_3() {
    let head = ListNode::new2(
        7,
        ListNode::new2(7, ListNode::new2(7, ListNode::new2(7, None))),
    );
    let val = 7;
    let res = Solution::remove_elements(head, val);
    assert_eq!(ListNode::to_vec(res), []);
}
