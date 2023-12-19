use leetcode_exercise::leetcode::editor::en::_105_construct_binary_tree_from_preorder_and_inorder_traversal::Solution;

#[test]
fn construct_binary_tree_from_preorder_and_inorder_traversal() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let root = Solution::build_tree(preorder, inorder);
    println!("{:#?}", root);
    // Some(
    //     RefCell {
    //         value: TreeNode {
    //             val: 3,
    //             left: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 9,
    //                         left: None,
    //                         right: None,
    //                     },
    //                 },
    //             ),
    //             right: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 20,
    //                         left: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: 15,
    //                                     left: None,
    //                                     right: None,
    //                                 },
    //                             },
    //                         ),
    //                         right: Some(
    //                             RefCell {
    //                                 value: TreeNode {
    //                                     val: 7,
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

    let preorder = vec![1, 2];
    let inorder = vec![2, 1];
    let root = Solution::build_tree(preorder, inorder);
    println!("{:#?}", root);
    // Some(
    //     RefCell {
    //         value: TreeNode {
    //             val: 1,
    //             left: Some(
    //                 RefCell {
    //                     value: TreeNode {
    //                         val: 2,
    //                         left: None,
    //                         right: None,
    //                     },
    //                 },
    //             ),
    //             right: None,
    //         },
    //     },
    // )
}
