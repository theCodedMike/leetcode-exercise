use leetcode_exercise::leetcode::editor::cn::_114_flatten_binary_tree_to_linked_list::{
    Solution, TreeNode,
};

#[test]
fn flatten_binary_tree_to_linked_list() {
    //    root            expected
    //
    //      1                1
    //     /                  \
    //    2                    2
    //   /                      \
    //  3                        3

    let mut root = TreeNode::new2(
        1,
        TreeNode::new2(2, TreeNode::new2(3, None, None), None),
        None,
    );
    Solution::flatten(&mut root);
    println!("{:#?}", root);
    // Some(
    //     RefCell {
    //         value: TreeNode {
    //             val: 1,
    //             left: None,
    //             right: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 2,
    //                         left: None,
    //                         right: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: 3,
    //                                     left: None,
    //                                     right: None,
    //                                 },
    //                             },
    //                         ),
    //                     },
    //                 },
    //             ),
    //         },
    //     },
    // )
}

#[test]
fn flatten_binary_tree_to_linked_list2() {
    //   root                 expected
    //
    //     4                    4
    //   /   \                   \
    //  1     5                   1
    //   \                         \
    //    2                         2
    //     \                         \
    //      3                         3
    //                                 \
    //                                  5

    let mut root = TreeNode::new2(
        4,
        TreeNode::new2(
            1,
            None,
            TreeNode::new2(2, None, TreeNode::new2(3, None, None)),
        ),
        TreeNode::new2(5, None, None),
    );
    Solution::flatten(&mut root);
    println!("{:#?}", root);
    // Some(
    //     RefCell {
    //         value: TreeNode {
    //             val: 4,
    //             left: None,
    //             right: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 1,
    //                         left: None,
    //                         right: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: 2,
    //                                     left: None,
    //                                     right: Some(
    //                                         RefCell {
    //                                             value: TreeNode {
    //                                                 val: 3,
    //                                                 left: None,
    //                                                 right: Some(
    //                                                     RefCell {
    //                                                         value: TreeNode {
    //                                                             val: 5,
    //                                                             left: None,
    //                                                             right: None,
    //                                                         },
    //                                                     },
    //                                                 ),
    //                                             },
    //                                         },
    //                                     ),
    //                                 },
    //                             },
    //                         ),
    //                     },
    //                 },
    //             ),
    //         },
    //     },
    // )
}
