use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_104_maximum_depth_of_binary_tree::Solution;

#[test]
fn maximum_depth_of_binary_tree_1() {
    //      3
    //    /  \
    //   9   20
    //      /  \
    //    15    7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new2(9),
        TreeNode::new_with_children(20, TreeNode::new2(15), TreeNode::new2(7)),
    );
    let depth = Solution::max_depth(root);
    assert_eq!(depth, 3);
}

#[test]
fn maximum_depth_of_binary_tree_2() {
    //      1
    //       \
    //        2
    let root = TreeNode::new_with_right(1, TreeNode::new2(2));
    let depth = Solution::max_depth(root);
    assert_eq!(depth, 2);
}
