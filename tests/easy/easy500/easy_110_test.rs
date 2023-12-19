use leetcode_exercise::leetcode::editor::en::_110_balanced_binary_tree::{Solution, TreeNode};

#[test]
fn balanced_binary_tree() {
    let root = TreeNode::new2(
        3,
        TreeNode::new2(9, None, None),
        TreeNode::new2(
            20,
            TreeNode::new2(15, None, None),
            TreeNode::new2(7, None, None),
        ),
    );
    let balanced = Solution::is_balanced(root);
    assert_eq!(balanced, true);
}
