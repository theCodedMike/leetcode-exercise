use leetcode_exercise::leetcode::editor::en::_145_binary_tree_postorder_traversal::Solution;
use leetcode_exercise::TreeNode;

#[test]
fn binary_tree_postorder_traversal() {
    //       3
    //    /    \
    //   9      4
    //  / \    / \
    // 0  10  5   7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(9, TreeNode::new(0), TreeNode::new(10)),
        TreeNode::new_with_children(4, TreeNode::new(5), TreeNode::new(7)),
    );
    let res = Solution::postorder_traversal(root);
    assert_eq!(res, [0, 10, 9, 5, 7, 4, 3]);
}
