use leetcode_exercise::binary_tree::safe::{
    in_order_recur, post_order_recur, pre_order_recur, TreeNode,
};
use leetcode_exercise::leetcode::editor::en::_538_convert_b_s_t_to_greater_tree::Solution;

#[test]
fn convert_bst_to_greater_tree_1() {
    //        4                              30
    //     /     \                         /    \
    //    1       6                      36      21
    //   / \     / \          =>        /  \    /  \
    //  0   2   5   7                  36  35  26  15
    //       \       \                      \       \
    //        3       8                     33       8
    let root = TreeNode::new_with_children(
        4,
        TreeNode::new_with_children(
            1,
            TreeNode::new2(0),
            TreeNode::new_with_right(2, TreeNode::new2(3)),
        ),
        TreeNode::new_with_children(
            6,
            TreeNode::new2(5),
            TreeNode::new_with_right(7, TreeNode::new2(8)),
        ),
    );
    let res = Solution::convert_bst(root);
    assert_eq!(
        pre_order_recur(res.clone()),
        [30, 36, 36, 35, 33, 21, 26, 15, 8]
    );
    assert_eq!(
        in_order_recur(res.clone()),
        [36, 36, 35, 33, 30, 26, 21, 15, 8]
    );
    assert_eq!(post_order_recur(res), [36, 33, 35, 36, 26, 8, 15, 21, 30]);
}

#[test]
fn convert_bst_to_greater_tree_2() {
    //   0                      1
    //    \         =>           \
    //     1                      1
    let root = TreeNode::new_with_right(0, TreeNode::new2(1));
    let res = Solution::convert_bst(root);
    assert_eq!(pre_order_recur(res.clone()), [1, 1]);
    assert_eq!(in_order_recur(res.clone()), [1, 1]);
    assert_eq!(post_order_recur(res), [1, 1]);
}

#[test]
fn convert_bst_to_greater_tree_3() {
    //   1                      3
    //  / \         =>         / \
    // 0   2                  3   2
    let root = TreeNode::new_with_children(1, TreeNode::new2(0), TreeNode::new2(2));
    let res = Solution::convert_bst(root);
    assert_eq!(pre_order_recur(res.clone()), [3, 3, 2]);
    assert_eq!(in_order_recur(res.clone()), [3, 3, 2]);
    assert_eq!(post_order_recur(res), [3, 2, 3]);
}

#[test]
fn convert_bst_to_greater_tree_4() {
    //     3                      7
    //    / \         =>         / \
    //   2   4                  9   4
    //  /                      /
    // 1                     10
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_left(2, TreeNode::new2(1)),
        TreeNode::new2(4),
    );
    let res = Solution::convert_bst(root);
    assert_eq!(pre_order_recur(res.clone()), [7, 9, 10, 4]);
    assert_eq!(in_order_recur(res.clone()), [10, 9, 7, 4]);
    assert_eq!(post_order_recur(res), [10, 9, 4, 7]);
}
