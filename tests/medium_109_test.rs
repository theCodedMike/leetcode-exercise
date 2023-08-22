use leetcode_exercise::leetcode::editor::en::_109_convert_sorted_list_to_binary_search_tree::{
    ListNode, Solution,
};

#[test]
fn convert_sorted_list_to_binary_search_tree() {
    let head = ListNode::new2(
        -10,
        ListNode::new2(
            -3,
            ListNode::new2(0, ListNode::new2(5, ListNode::new2(9, None))),
        ),
    );
    let root = Solution::sorted_list_to_bst(head);
    println!("{:#?}", root);
    // Some(
    //     RefCell {
    //         value: TreeNode {
    //             val: 0,
    //             left: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: -3,
    //                         left: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: -10,
    //                                     left: None,
    //                                     right: None,
    //                                 },
    //                             },
    //                         ),
    //                         right: None,
    //                     },
    //                 },
    //             ),
    //             right: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 9,
    //                         left: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: 5,
    //                                     left: None,
    //                                     right: None,
    //                                 },
    //                             },
    //                         ),
    //                         right: None,
    //                     },
    //                 },
    //             ),
    //         },
    //     },
    // )
}
