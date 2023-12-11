use leetcode_exercise::leetcode::editor::en::_102_binary_tree_level_order_traversal::Solution;
use leetcode_exercise::TreeNode;

#[test]
fn binary_tree_level_order_traversal_1() {
    let root = TreeNode::new2(
        3,
        TreeNode::new2(9, None, None),
        TreeNode::new2(
            20,
            TreeNode::new2(15, None, None),
            TreeNode::new2(7, None, None),
        ),
    );
    let res = Solution::level_order(root);
    assert_eq!(res, vec![vec![3], vec![9, 20], vec![15, 7]]);
}

#[test]
fn binary_tree_level_order_traversal_2() {
    let root = TreeNode::new2(1, None, None);
    let res = Solution::level_order(root);
    assert_eq!(res, vec![[1]]);
}

#[test]
fn binary_tree_level_order_traversal_3() {
    let root = None;
    let res = Solution::level_order(root);
    assert_eq!(res.is_empty(), true);
}
