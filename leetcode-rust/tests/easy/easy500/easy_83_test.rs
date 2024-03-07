use leetcode_rust::leetcode::editor::cn::_83_remove_duplicates_from_sorted_list::{
    ListNode, Solution,
};

/// cargo test -- --show-output remove_duplicates_from_sorted_list
#[test]
fn remove_duplicates_from_sorted_list() {
    //let _3_node = ListNode::new(3, None);
    let _1_node = ListNode::new(1, None);

    //let _3_node = ListNode::new(3, Some(Box::new(_3_node)));

    //let _2_node = ListNode::new(2, Some(Box::new(_3_node)));

    let _1_node = ListNode::new(1, Some(Box::new(_1_node)));

    let _1_node = ListNode::new(1, Some(Box::new(_1_node)));

    let head = Some(Box::new(_1_node));

    let option = Solution::delete_duplicates(head);

    println!("{:?}", option); // Some(ListNode { val: 1, next: None })
}
