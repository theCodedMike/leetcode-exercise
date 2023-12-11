use leetcode_exercise::leetcode::editor::en::_94_binary_tree_inorder_traversal::Solution;
use leetcode_exercise::TreeNode;

#[test]
fn binary_tree_inorder_traversal_1() {
    //  1
    //   \
    //    2
    //   /
    //  3
    let root = TreeNode::new2(
        1,
        None,
        TreeNode::new2(2, TreeNode::new2(3, None, None), None),
    );
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
    let root = TreeNode::new2(
        2,
        TreeNode::new2(3, TreeNode::new2(1, None, None), None),
        None,
    );
    let res = Solution::inorder_traversal(root);
    assert_eq!(res, [1, 3, 2]);
}
