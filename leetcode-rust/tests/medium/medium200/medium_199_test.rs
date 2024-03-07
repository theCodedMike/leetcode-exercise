use leetcode_rust::binary_tree::safe::TreeNode;
use leetcode_rust::leetcode::editor::cn::_199_binary_tree_right_side_view::Solution;

#[test]
fn binary_tree_right_side_view_1() {
    //   1
    //  / \
    // 2   3
    //  \   \
    //   5   4
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_right(2, TreeNode::new2(5)),
        TreeNode::new_with_right(3, TreeNode::new2(4)),
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3, 4]);
}

#[test]
fn binary_tree_right_side_view_2() {
    //   1
    //    \
    //     3
    let root = TreeNode::new_with_right(1, TreeNode::new2(3));
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3]);
}

#[test]
fn binary_tree_right_side_view_3() {
    //   1
    //  /
    // 2
    let root = TreeNode::new_with_left(1, TreeNode::new2(2));
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 2]);
}

#[test]
fn binary_tree_right_side_view_4() {
    //     1
    //    / \
    //   2   3
    //  /
    // 4
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_left(2, TreeNode::new2(4)),
        TreeNode::new2(3),
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [1, 3, 4]);
}

#[test]
fn binary_tree_right_side_view_5() {
    //   6
    //  /
    // 1
    //  \
    //   3
    //  / \
    // 2   5
    //      \
    //       4
    let root = TreeNode::new_with_left(
        6,
        TreeNode::new_with_right(
            1,
            TreeNode::new_with_children(
                3,
                TreeNode::new2(2),
                TreeNode::new_with_right(5, TreeNode::new2(4)),
            ),
        ),
    );
    let res = Solution::right_side_view(root);
    assert_eq!(res, [6, 1, 3, 5, 4]);
}
