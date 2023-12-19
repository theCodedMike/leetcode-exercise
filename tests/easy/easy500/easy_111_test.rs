use leetcode_exercise::leetcode::editor::en::_111_minimum_depth_of_binary_tree::{
    Solution, TreeNode,
};

#[test]
fn minimum_depth_of_binary_tree() {
    //     3
    //    / \
    //   9   20
    //      /  \
    //     15   7
    let root = TreeNode::new2(
        3,
        TreeNode::new2(9, None, None),
        TreeNode::new2(
            20,
            TreeNode::new2(15, None, None),
            TreeNode::new2(7, None, None),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 2);
}

#[test]
fn minimum_depth_of_binary_tree2() {
    //  2
    //   \
    //    3
    //     \
    //      4
    //       \
    //        5
    //         \
    //          6
    let root = TreeNode::new2(
        2,
        None,
        TreeNode::new2(
            3,
            None,
            TreeNode::new2(
                4,
                None,
                TreeNode::new2(5, None, TreeNode::new2(6, None, None)),
            ),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 5);
}

#[test]
fn minimum_depth_of_binary_tree3() {
    //       -9
    //    /      \
    //  -3        2
    //    \      / \
    //     4    4   0
    //    /    /
    //  -6   -5
    let root = TreeNode::new2(
        -9,
        TreeNode::new2(
            -3,
            None,
            TreeNode::new2(4, TreeNode::new2(-6, None, None), None),
        ),
        TreeNode::new2(
            2,
            TreeNode::new2(4, TreeNode::new2(-5, None, None), None),
            TreeNode::new2(0, None, None),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 3);
}

#[test]
fn minimum_depth_of_binary_tree4() {
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
    let root = TreeNode::new2(
        1,
        None,
        TreeNode::new2(
            -9,
            None,
            TreeNode::new2(
                8,
                TreeNode::new2(4, None, None),
                TreeNode::new2(
                    -3,
                    TreeNode::new2(
                        -3,
                        TreeNode::new2(
                            -6,
                            None,
                            TreeNode::new2(
                                -6,
                                TreeNode::new2(-4, None, None),
                                TreeNode::new2(-9, TreeNode::new2(6, None, None), None),
                            ),
                        ),
                        None,
                    ),
                    None,
                ),
            ),
        ),
    );

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 4);
}
