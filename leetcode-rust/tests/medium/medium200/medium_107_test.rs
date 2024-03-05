use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_107_binary_tree_level_order_traversal_i_i::Solution;
#[test]
fn binary_tree_level_order_traversal_ii_1() {
    //      3
    //    /   \
    //  9      20
    //        /  \
    //      15    7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new2(9),
        TreeNode::new_with_children(20, TreeNode::new2(15), TreeNode::new2(7)),
    );
    let res = Solution::level_order_bottom(root);
    assert_eq!(res, vec![vec![15, 7], vec![9, 20], vec![3]]);
}

#[test]
fn binary_tree_level_order_traversal_ii_2() {
    //  1
    let root = TreeNode::new2(1);
    let res = Solution::level_order_bottom(root);
    assert_eq!(res, vec![vec![1]]);
}

#[test]
fn binary_tree_level_order_traversal_ii_3() {
    let root = None;
    let res = Solution::level_order_bottom(root);
    assert_eq!(res.is_empty(), true);
}

#[test]
fn binary_tree_level_order_traversal_ii_4() {
    //          3
    //        /   \
    //       4     5
    //     /  \
    //   -7   -6
    //   /    /
    // -7   -5
    //      /
    //    -4
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(
            4,
            TreeNode::new_with_left(-7, TreeNode::new2(-7)),
            TreeNode::new_with_left(-6, TreeNode::new_with_left(-5, TreeNode::new2(-4))),
        ),
        TreeNode::new2(5),
    );
    let res = Solution::level_order_bottom(root);
    assert_eq!(
        res,
        vec![vec![-4], vec![-7, -5], vec![-7, -6], vec![4, 5], vec![3]]
    );
}
