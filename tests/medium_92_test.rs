use leetcode_exercise::leetcode::editor::en::_92_reverse_linked_list_i_i::{ListNode, Solution};

#[test]
fn reverse_linked_list_ii() {
    // [5] 1 1
    let head = Some(Box::new(ListNode::new(5)));
    let left = 1;
    let right = 1;
    let res = Solution::reverse_between(head.clone(), left, right);
    assert_eq!(res, head);
}

#[test]
fn reverse_linked_list_ii_2() {
    // [3, 5] 2 2
    let five = Some(Box::new(ListNode::new(5)));
    let head = Some(Box::new(ListNode::new2(3, five)));
    let left = 2;
    let right = 2;
    let res = Solution::reverse_between(head.clone(), left, right);
    assert_eq!(res, head);
}

#[test]
fn reverse_linked_list_ii_3() {
    // [3, 5] 1 2
    let five = Some(Box::new(ListNode::new(5)));
    let head = Some(Box::new(ListNode::new2(3, five)));
    let left = 1;
    let right = 2;
    let res = Solution::reverse_between(head, left, right);
    assert_eq!(
        res,
        Some(Box::new(ListNode::new2(
            5,
            Some(Box::new(ListNode::new(3)))
        )))
    );
}

#[test]
fn reverse_linked_list_ii_4() {
    // [1, 2, 3] 2 3
    let five = Some(Box::new(ListNode::new(5)));
    let head = Some(Box::new(ListNode::new2(3, five)));
    let left = 2;
    let right = 2;
    let _res = Solution::reverse_between(head, left, right);
}
