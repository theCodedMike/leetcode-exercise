use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_98_validate_binary_search_tree::Solution;

#[test]
fn validate_binary_search_tree_1() {
    //   2
    //  / \
    // 1   3
    let root = TreeNode::new_with_children(2, TreeNode::new2(1), TreeNode::new2(3));
    let res = Solution::is_valid_bst(root);
    assert_eq!(res, true);
}

#[test]
fn validate_binary_search_tree_2() {
    //   5
    //  / \
    // 1   4
    //    / \
    //   3   6
    let root = TreeNode::new_with_children(
        5,
        TreeNode::new2(1),
        TreeNode::new_with_children(4, TreeNode::new2(3), TreeNode::new2(6)),
    );
    let res = Solution::is_valid_bst(root);
    assert_eq!(res, false);
}

#[test]
fn validate_binary_search_tree_3() {
    //   5
    //  / \
    // 4   6
    //    / \
    //   3   7
    let root = TreeNode::new_with_children(
        5,
        TreeNode::new2(4),
        TreeNode::new_with_children(6, TreeNode::new2(3), TreeNode::new2(7)),
    );
    let res = Solution::is_valid_bst(root);
    assert_eq!(res, false);
}

#[test]
fn validate_binary_search_tree_4() {
    //      32
    //     /  \
    //    26  47
    //   /      \
    // 19       56
    //  \
    //  27
    let root = TreeNode::new_with_children(
        5,
        TreeNode::new2(4),
        TreeNode::new_with_children(6, TreeNode::new2(3), TreeNode::new2(7)),
    );
    let res = Solution::is_valid_bst(root);
    assert_eq!(res, false);
}

#[test]
fn validate_binary_search_tree_5() {
    //      3
    //    /   \
    //   1     5
    //  / \   / \
    // 0   2 4   6
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(1, TreeNode::new2(0), TreeNode::new2(2)),
        TreeNode::new_with_children(5, TreeNode::new2(4), TreeNode::new2(6)),
    );
    let res = Solution::is_valid_bst(root);
    assert_eq!(res, true);
}
