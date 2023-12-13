use leetcode_exercise::leetcode::editor::en::_145_binary_tree_postorder_traversal::Solution;
use leetcode_exercise::TreeNode;

#[test]
fn binary_tree_postorder_traversal() {
    //       3
    //    /    \
    //   9      4
    //  / \    / \
    // 0  10  5   7
    let root = TreeNode::new2(
        3,
        TreeNode::new2(
            9,
            TreeNode::new2(0, None, None),
            TreeNode::new2(10, None, None),
        ),
        TreeNode::new2(
            4,
            TreeNode::new2(5, None, None),
            TreeNode::new2(7, None, None),
        ),
    );
    let res = Solution::postorder_traversal(root);
    assert_eq!(res, [0, 10, 9, 5, 7, 4, 3]);
}
