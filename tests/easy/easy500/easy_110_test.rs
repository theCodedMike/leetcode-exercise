use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::en::_110_balanced_binary_tree::Solution;

#[test]
fn balanced_binary_tree_1() {
    //      3
    //    /  \
    //   9   20
    //      /  \
    //     15   7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new2(9),
        TreeNode::new_with_children(20, TreeNode::new2(15), TreeNode::new2(7)),
    );
    let balanced = Solution::is_balanced(root);
    assert_eq!(balanced, true);
}

#[test]
fn balanced_binary_tree_2() {
    //       1
    //      / \
    //     2   2
    //    / \
    //   3   3
    //  / \
    // 4   4
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(
            2,
            TreeNode::new_with_children(3, TreeNode::new2(4), TreeNode::new2(4)),
            TreeNode::new2(3),
        ),
        TreeNode::new2(2),
    );
    let balanced = Solution::is_balanced(root);
    assert_eq!(balanced, false);
}

#[test]
fn balanced_binary_tree_3() {
    let root = None;
    let balanced = Solution::is_balanced(root);
    assert_eq!(balanced, true);
}

#[test]
fn balanced_binary_tree_4() {
    //        1
    //       / \
    //      2   2
    //     /     \
    //    3       3
    //   /         \
    //  4           4

    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_left(2, TreeNode::new_with_left(3, TreeNode::new2(4))),
        TreeNode::new_with_right(2, TreeNode::new_with_right(3, TreeNode::new2(4))),
    );
    let balanced = Solution::is_balanced(root);
    assert_eq!(balanced, false);
}
