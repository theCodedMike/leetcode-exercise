use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::en::_145_binary_tree_postorder_traversal::Solution;

#[test]
fn binary_tree_postorder_traversal() {
    //       3
    //    /    \
    //   9      4
    //  / \    / \
    // 0  10  5   7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(9, TreeNode::new2(0), TreeNode::new2(10)),
        TreeNode::new_with_children(4, TreeNode::new2(5), TreeNode::new2(7)),
    );
    let res = Solution::postorder_traversal(root);
    assert_eq!(res, [0, 10, 9, 5, 7, 4, 3]);
}
