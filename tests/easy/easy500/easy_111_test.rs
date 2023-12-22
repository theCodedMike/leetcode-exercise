use leetcode_exercise::binary_tree::TreeNode;
use leetcode_exercise::leetcode::editor::en::_111_minimum_depth_of_binary_tree::Solution;

#[test]
fn minimum_depth_of_binary_tree_1() {
    //     3
    //    / \
    //   9  20
    //     /  \
    //    15   7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new(9),
        TreeNode::new_with_children(20, TreeNode::new(15), TreeNode::new(7)),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 2);
}

#[test]
fn minimum_depth_of_binary_tree_2() {
    //  2
    //   \
    //    3
    //     \
    //      4
    //       \
    //        5
    //         \
    //          6
    let root = TreeNode::new_with_right(
        2,
        TreeNode::new_with_right(
            3,
            TreeNode::new_with_right(4, TreeNode::new_with_right(5, TreeNode::new(6))),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 5);
}

#[test]
fn minimum_depth_of_binary_tree_3() {
    //       -9
    //    /      \
    //  -3        2
    //    \      / \
    //     4    4   0
    //    /    /
    //  -6   -5
    let root = TreeNode::new_with_children(
        -9,
        TreeNode::new_with_right(-3, TreeNode::new_with_left(4, TreeNode::new(-6))),
        TreeNode::new_with_children(
            2,
            TreeNode::new_with_left(4, TreeNode::new(-5)),
            TreeNode::new(0),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 3);
}

#[test]
fn minimum_depth_of_binary_tree_4() {
    //    1
    //     \
    //     -9
    //       \
    //        8
    //       / \
    //      4  -3
    //         /
    //        -3
    //        /
    //       -6
    //         \
    //         -6
    //         / \
    //       -4  -9
    //           /
    //          6
    let root = TreeNode::new_with_right(
        1,
        TreeNode::new_with_right(
            -9,
            TreeNode::new_with_children(
                8,
                TreeNode::new(4),
                TreeNode::new_with_left(
                    -3,
                    TreeNode::new_with_left(
                        -3,
                        TreeNode::new_with_right(
                            -6,
                            TreeNode::new_with_children(
                                -6,
                                TreeNode::new(-4),
                                TreeNode::new_with_left(-9, TreeNode::new(6)),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 4);
}
