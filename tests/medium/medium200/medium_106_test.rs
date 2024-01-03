use leetcode_exercise::leetcode::editor::en::_106_construct_binary_tree_from_inorder_and_postorder_traversal::Solution;

#[test]
fn construct_binary_tree_from_inorder_and_postorder_traversal_1() {
    //     3
    //    / \
    //   9   20
    //      /  \
    //     15   7
    let postorder = vec![9, 15, 7, 20, 3];
    let inorder = vec![9, 3, 15, 20, 7];
    let root = Solution::build_tree(inorder, postorder);
    println!("{:?}", root);
}

#[test]
fn construct_binary_tree_from_inorder_and_postorder_traversal_2() {
    //    1
    //   /
    //  2
    let postorder = vec![2, 1];
    let inorder = vec![2, 1];
    let root = Solution::build_tree(inorder, postorder);
    println!("{:?}", root);
}

#[test]
fn construct_binary_tree_from_inorder_and_postorder_traversal_3() {
    //     3
    //    / \
    //   9  20
    //  / \   \
    // 15 10   7
    //        / \
    //       5   8
    //            \
    //             4
    let postorder = vec![15, 10, 9, 5, 4, 8, 7, 20, 3];
    let inorder = vec![15, 9, 10, 3, 20, 5, 7, 8, 4];
    let root = Solution::build_tree(inorder, postorder);
    println!("{:?}", root);
}
