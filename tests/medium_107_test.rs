use leetcode_exercise::leetcode::editor::en::_107_binary_tree_level_order_traversal_i_i::Solution;
use leetcode_exercise::TreeNode;
#[test]
fn binary_tree_level_order_traversal_ii_1() {
    //      3
    //    /   \
    //  9      20
    //        /  \
    //      15    7
    let root = TreeNode::new2(
        3,
        TreeNode::new2(9, None, None),
        TreeNode::new2(
            20,
            TreeNode::new2(15, None, None),
            TreeNode::new2(7, None, None),
        ),
    );
    let res = Solution::level_order_bottom(root);
    assert_eq!(res, vec![vec![15, 7], vec![9, 20], vec![3]]);
}

#[test]
fn binary_tree_level_order_traversal_ii_2() {
    //  1
    let root = TreeNode::new2(1, None, None);
    let res = Solution::level_order_bottom(root);
    assert_eq!(res, vec![vec![1]]);
}

#[test]
fn binary_tree_level_order_traversal_ii_3() {
    //  1
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
    let root = TreeNode::new2(
        3,
        TreeNode::new2(
            4,
            TreeNode::new2(-7, TreeNode::new2(-7, None, None), None),
            TreeNode::new2(
                -6,
                TreeNode::new2(-5, TreeNode::new2(-4, None, None), None),
                None,
            ),
        ),
        TreeNode::new2(5, None, None),
    );
    let res = Solution::level_order_bottom(root);
    assert_eq!(
        res,
        vec![vec![-4], vec![-7, -5], vec![-7, -6], vec![4, 5], vec![3]]
    );
}
