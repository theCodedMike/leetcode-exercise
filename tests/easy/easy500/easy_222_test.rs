use leetcode_exercise::binary_tree::TreeNode;
use leetcode_exercise::leetcode::editor::en::_222_count_complete_tree_nodes::Solution;
#[test]
fn count_complete_tree_nodes_1() {
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(2, TreeNode::new(4), TreeNode::new(5)),
        TreeNode::new_with_left(3, TreeNode::new(6)),
    );
    let res = Solution::count_nodes(root);
    assert_eq!(res, 6);
}

#[test]
fn count_complete_tree_nodes_2() {
    let root = None;
    let res = Solution::count_nodes(root);
    assert_eq!(res, 0);
}

#[test]
fn count_complete_tree_nodes_3() {
    let root = TreeNode::new(1);
    let res = Solution::count_nodes(root);
    assert_eq!(res, 1);
}