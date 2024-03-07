use leetcode_rust::binary_tree::safe::TreeNode;
use leetcode_rust::leetcode::editor::cn::_113_path_sum_i_i::Solution;

#[test]
fn path_sum_ii() {
    //                   1
    //              /        \
    //            0           1
    //          /   \       /   \
    //         1     2     0    -1
    //        / \   / \   / \   / \
    //       0   1 -1  0 -1  0 1   0
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(
            0,
            TreeNode::new_with_children(1, TreeNode::new2(0), TreeNode::new2(1)),
            TreeNode::new_with_children(2, TreeNode::new2(-1), TreeNode::new2(0)),
        ),
        TreeNode::new_with_children(
            1,
            TreeNode::new_with_children(0, TreeNode::new2(-1), TreeNode::new2(0)),
            TreeNode::new_with_children(-1, TreeNode::new2(1), TreeNode::new2(0)),
        ),
    );
    let target_sum = 2;
    let res = Solution::path_sum(root, target_sum);
    assert_eq!(
        res,
        [[1, 0, 1, 0], [1, 0, 2, -1], [1, 1, 0, 0], [1, 1, -1, 1]]
    );
}
