use leetcode_rust::binary_tree::safe::{BinaryTree, TreeNode};
use leetcode_rust::leetcode::editor::cn::_701_insert_into_a_binary_search_tree::Solution;
use leetcode_rust::Traverse;

#[test]
fn insert_into_a_binary_search_tree_1() {
    //      4                        4
    //    /  \         5          /    \
    //   2    7       ——>        2      7
    //  / \                     / \    /
    // 1   3                   1   3  5
    let root = TreeNode::new_with_children(
        4,
        TreeNode::new_with_children(2, TreeNode::new2(1), TreeNode::new2(3)),
        TreeNode::new2(7),
    );
    let val = 5;
    let res = Solution::insert_into_bst(root, val);

    assert_eq!(BinaryTree::pre_order_recur(res.clone()), [4, 2, 1, 3, 7, 5]);
    assert_eq!(BinaryTree::in_order_recur(res.clone()), [1, 2, 3, 4, 5, 7]);
    assert_eq!(BinaryTree::post_order_recur(res), [1, 3, 2, 5, 7, 4]);
}

#[test]
fn insert_into_a_binary_search_tree_2() {
    //        40                         40
    //      /   \         25           /    \
    //    20     60       ——>        20     60
    //   / \    /  \                / \    /  \
    // 10  30  50  70             10  30  50  70
    //                                /
    //                              25
    let root = TreeNode::new_with_children(
        40,
        TreeNode::new_with_children(20, TreeNode::new2(10), TreeNode::new2(30)),
        TreeNode::new_with_children(60, TreeNode::new2(50), TreeNode::new2(70)),
    );
    let val = 25;
    let res = Solution::insert_into_bst(root, val);

    assert_eq!(
        BinaryTree::pre_order_recur(res.clone()),
        [40, 20, 10, 30, 25, 60, 50, 70]
    );
    assert_eq!(
        BinaryTree::in_order_recur(res.clone()),
        [10, 20, 25, 30, 40, 50, 60, 70]
    );
    assert_eq!(
        BinaryTree::post_order_recur(res),
        [10, 25, 30, 20, 50, 70, 60, 40]
    );
}
