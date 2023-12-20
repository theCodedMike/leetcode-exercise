use leetcode_exercise::binary_tree::TreeNode;
use leetcode_exercise::leetcode::editor::en::_101_symmetric_tree::Solution;
#[test]
fn symmetric_tree_1() {
    //        1
    //      /  \
    //    2     2
    //   / \   / \
    //  3   4 4   3
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(2, TreeNode::new(3), TreeNode::new(4)),
        TreeNode::new_with_children(2, TreeNode::new(4), TreeNode::new(3)),
    );
    let res = Solution::is_symmetric(root);
    assert_eq!(res, true);
}

#[test]
fn symmetric_tree_2() {
    //        1
    //      /  \
    //    2     2
    //     \     \
    //      3     3
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_right(2, TreeNode::new(3)),
        TreeNode::new_with_right(2, TreeNode::new(3)),
    );
    let res = Solution::is_symmetric(root);
    assert_eq!(res, false);
}

#[test]
fn symmetric_tree_3() {
    //    1
    //     \
    //      2
    let root = TreeNode::new_with_right(1, TreeNode::new(2));
    let res = Solution::is_symmetric(root);
    assert_eq!(res, false);
}

#[test]
fn symmetric_tree_4() {
    //    1
    //   /
    //  2
    let root = TreeNode::new_with_left(1, TreeNode::new(2));
    let res = Solution::is_symmetric(root);
    assert_eq!(res, false);
}
