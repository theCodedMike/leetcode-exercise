use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_530_minimum_absolute_difference_in_b_s_t::Solution;

#[test]
fn minimum_absolute_difference_in_bst_1() {
    //       4
    //      / \
    //     2   6
    //    / \
    //   1   3
    let root = TreeNode::new_with_children(
        4,
        TreeNode::new_with_children(2, TreeNode::new2(1), TreeNode::new2(3)),
        TreeNode::new2(6),
    );
    let diff = Solution::get_minimum_difference(root);
    assert_eq!(diff, 1);
}

#[test]
fn minimum_absolute_difference_in_bst_2() {
    //    1
    //   / \
    //  0  48
    //    /  \
    //  12    49
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new2(0),
        TreeNode::new_with_children(48, TreeNode::new2(12), TreeNode::new2(49)),
    );
    let diff = Solution::get_minimum_difference(root);
    assert_eq!(diff, 1);
}

#[test]
fn minimum_absolute_difference_in_bst_3() {
    //    600
    //   /   \
    //  424  612
    //    \    \
    //    499  689
    let root = TreeNode::new_with_children(
        600,
        TreeNode::new_with_right(424, TreeNode::new2(499)),
        TreeNode::new_with_right(612, TreeNode::new2(689)),
    );
    let diff = Solution::get_minimum_difference(root);
    assert_eq!(diff, 12);
}

#[test]
fn minimum_absolute_difference_in_bst_5() {
    //       1564
    //      /    \
    //   1434   3048
    //   /        \
    //  1        3184
    let root = TreeNode::new_with_children(
        1564,
        TreeNode::new_with_left(1434, TreeNode::new2(1)),
        TreeNode::new_with_right(3048, TreeNode::new2(3184)),
    );
    let diff = Solution::get_minimum_difference(root);
    assert_eq!(diff, 130);
}
