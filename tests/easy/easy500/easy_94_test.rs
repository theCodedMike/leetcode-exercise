use leetcode_exercise::binary_tree::TreeNode;
use leetcode_exercise::leetcode::editor::en::_94_binary_tree_inorder_traversal::Solution;

#[test]
fn binary_tree_inorder_traversal_1() {
    //  1
    //   \
    //    2
    //   /
    //  3
    let root = TreeNode::new_with_right(1, TreeNode::new_with_left(2, TreeNode::new(3)));
    let res = Solution::inorder_traversal(root);
    assert_eq!(res, [1, 3, 2]);
}

#[test]
fn binary_tree_inorder_traversal_2() {
    //      2
    //     /
    //    3
    //   /
    //  1
    let root = TreeNode::new_with_left(2, TreeNode::new_with_left(3, TreeNode::new(1)));
    let res = Solution::inorder_traversal(root);
    assert_eq!(res, [1, 3, 2]);
}
