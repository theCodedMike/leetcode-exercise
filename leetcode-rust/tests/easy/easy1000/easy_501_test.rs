use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_501_find_mode_in_binary_search_tree::Solution;
#[test]
fn find_mode_in_binary_search_tree_1() {
    //  1
    //   \
    //    2
    //   /
    //  2
    let root = TreeNode::new_with_right(1, TreeNode::new_with_left(2, TreeNode::new2(2)));
    let res = Solution::find_mode(root);
    assert_eq!(res, [2]);
}

#[test]
fn find_mode_in_binary_search_tree_2() {
    //  0
    let root = TreeNode::new2(0);
    let res = Solution::find_mode(root);
    assert_eq!(res, [0]);
}

#[test]
fn find_mode_in_binary_search_tree_3() {
    //       6
    //    /    \
    //   2      8
    //  / \    / \
    // 0   4  7   9
    //    / \
    //   2   6
    let root = TreeNode::new_with_children(
        6,
        TreeNode::new_with_children(
            2,
            TreeNode::new2(0),
            TreeNode::new_with_children(4, TreeNode::new2(2), TreeNode::new2(6)),
        ),
        TreeNode::new_with_children(8, TreeNode::new2(7), TreeNode::new2(9)),
    );
    let res = Solution::find_mode(root);
    assert_eq!(res, [2, 6]);
}
