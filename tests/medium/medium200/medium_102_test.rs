use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::en::_102_binary_tree_level_order_traversal::Solution;

#[test]
fn binary_tree_level_order_traversal_1() {
    //     3
    //    / \
    //   9   20
    //      /  \
    //     15   7
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new2(9),
        TreeNode::new_with_children(20, TreeNode::new2(15), TreeNode::new2(7)),
    );
    let res = Solution::level_order(root);
    assert_eq!(res, vec![vec![3], vec![9, 20], vec![15, 7]]);
}

#[test]
fn binary_tree_level_order_traversal_2() {
    let root = TreeNode::new2(1);
    let res = Solution::level_order(root);
    assert_eq!(res, vec![[1]]);
}

#[test]
fn binary_tree_level_order_traversal_3() {
    let root = None;
    let res = Solution::level_order(root);
    assert_eq!(res.is_empty(), true);
}
