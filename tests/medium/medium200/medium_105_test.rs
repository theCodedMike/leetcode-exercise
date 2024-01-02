use leetcode_exercise::leetcode::editor::en::_105_construct_binary_tree_from_preorder_and_inorder_traversal::Solution;

#[test]
fn construct_binary_tree_from_preorder_and_inorder_traversal_1() {
    //     3
    //    / \
    //   9   20
    //      /  \
    //     15   7
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let root = Solution::build_tree(preorder, inorder);
    println!("{:#?}", root);
}

#[test]
fn construct_binary_tree_from_preorder_and_inorder_traversal_2() {
    //    1
    //   /
    //  2
    let preorder = vec![1, 2];
    let inorder = vec![2, 1];
    let root = Solution::build_tree(preorder, inorder);
    println!("{:#?}", root);
}

#[test]
fn construct_binary_tree_from_preorder_and_inorder_traversal_3() {
    //            3
    //         /    \
    //        9     20
    //       /     /  \
    //      8    15    7
    //     / \
    //    5  10
    //   /
    //  4
    let preorder = vec![3, 9, 8, 5, 4, 10, 20, 15, 7];
    let inorder = vec![4, 5, 8, 10, 9, 3, 15, 20, 7];
    let root = Solution::build_tree(preorder, inorder);
    println!("{:#?}", root);
}
