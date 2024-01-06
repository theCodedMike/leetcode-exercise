use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::en::_515_find_largest_value_in_each_tree_row::Solution;

#[test]
fn find_largest_value_in_each_tree_row_1() {
    //      1
    //     / \
    //    3   2
    //  /  \   \
    // 5    3   9
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(3, TreeNode::new2(5), TreeNode::new2(3)),
        TreeNode::new_with_right(2, TreeNode::new2(9)),
    );
    let res = Solution::largest_values(root);
    assert_eq!(res, [1, 3, 9]);
}

#[test]
fn find_largest_value_in_each_tree_row_2() {
    //      1
    //     / \
    //    2   3
    let root = TreeNode::new_with_children(1, TreeNode::new2(2), TreeNode::new2(3));
    let res = Solution::largest_values(root);
    assert_eq!(res, [1, 3]);
}
