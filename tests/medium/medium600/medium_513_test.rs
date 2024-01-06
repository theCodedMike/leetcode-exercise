use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::en::_513_find_bottom_left_tree_value::Solution;

#[test]
fn find_bottom_left_tree_value_1() {
    //    2
    //   / \
    //  1   3
    let root = TreeNode::new_with_children(2, TreeNode::new2(1), TreeNode::new2(3));
    let res = Solution::find_bottom_left_value(root);
    assert_eq!(res, 1);
}

#[test]
fn find_bottom_left_tree_value_2() {
    //      1
    //     / \
    //    2   3
    //   /   / \
    //  4   5   6
    //     /
    //    7
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_left(2, TreeNode::new2(4)),
        TreeNode::new_with_children(
            3,
            TreeNode::new_with_left(5, TreeNode::new2(7)),
            TreeNode::new2(6),
        ),
    );
    let res = Solution::find_bottom_left_value(root);
    assert_eq!(res, 7);
}

#[test]
fn find_bottom_left_tree_value_3() {
    //    2
    let root = TreeNode::new2(2);
    let res = Solution::find_bottom_left_value(root);
    assert_eq!(res, 2);
}
