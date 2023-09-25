use leetcode_exercise::leetcode::editor::en::_199_binary_tree_right_side_view::{
    Solution, TreeNode,
};

#[test]
fn binary_tree_right_side_view() {
    //   1
    //  / \
    // 2   3
    //  \   \
    //   5   4
    let root = TreeNode::new2(
        1,
        TreeNode::new2(2, None, TreeNode::new2(5, None, None)),
        TreeNode::new2(3, None, TreeNode::new2(4, None, None)),
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3, 4]);

    //   1
    //    \
    //     3
    let root = TreeNode::new2(1, None, TreeNode::new2(3, None, None));
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3]);

    //   1
    //  /
    // 2
    let root = TreeNode::new2(1, TreeNode::new2(2, None, None), None);
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 2]);

    //     1
    //    / \
    //   2   3
    //  /
    // 4
    let root = TreeNode::new2(
        1,
        TreeNode::new2(2, TreeNode::new2(4, None, None), None),
        TreeNode::new2(3, None, None),
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3, 4]);

    //   6
    //  /
    // 1
    //  \
    //   3
    //  / \
    // 2   5
    //      \
    //       4
    let root = TreeNode::new2(
        6,
        TreeNode::new2(
            1,
            None,
            TreeNode::new2(
                3,
                TreeNode::new2(2, None, None),
                TreeNode::new2(5, None, TreeNode::new2(4, None, None)),
            ),
        ),
        None,
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [6, 1, 3, 5, 4]);
}
