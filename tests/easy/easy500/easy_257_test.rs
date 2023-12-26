use leetcode_exercise::binary_tree::TreeNode;
use leetcode_exercise::leetcode::editor::en::_257_binary_tree_paths::Solution;

#[test]
fn binary_tree_paths_1() {
    //     1
    //    / \
    //   2   3
    //    \
    //     5
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_right(2, TreeNode::new(5)),
        TreeNode::new(3),
    );
    let res = Solution::binary_tree_paths(root);
    ["1->3".to_string(), "1->2->5".to_string()]
        .into_iter()
        .for_each(|ref expected_path| {
            assert_eq!(res.contains(expected_path), true);
        });
}

#[test]
fn binary_tree_paths_2() {
    //   1
    let root = TreeNode::new(1);
    let res = Solution::binary_tree_paths(root);
    assert_eq!(res, ["1"]);
}

#[test]
fn binary_tree_paths_3() {
    //      1
    //    /   \
    //   2     3
    //  / \   / \
    // 4   5 6   7
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(2, TreeNode::new(4), TreeNode::new(5)),
        TreeNode::new_with_children(3, TreeNode::new(6), TreeNode::new(7)),
    );
    let res = Solution::binary_tree_paths(root);
    [
        "1->2->4".to_string(),
        "1->2->5".to_string(),
        "1->3->6".to_string(),
        "1->3->7".to_string(),
    ]
    .into_iter()
    .for_each(|ref expected_path| {
        assert_eq!(res.contains(expected_path), true);
    });
}

#[test]
fn binary_tree_paths_4() {
    //             1
    //        /         \
    //       2           3
    //     /   \       /   \
    //    4     5     6     7
    //   / \   / \   / \   / \
    //  8   9 10 11 12 13 14 15
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(
            2,
            TreeNode::new_with_children(4, TreeNode::new(8), TreeNode::new(9)),
            TreeNode::new_with_children(5, TreeNode::new(10), TreeNode::new(11)),
        ),
        TreeNode::new_with_children(
            3,
            TreeNode::new_with_children(6, TreeNode::new(12), TreeNode::new(13)),
            TreeNode::new_with_children(7, TreeNode::new(14), TreeNode::new(15)),
        ),
    );
    let res = Solution::binary_tree_paths(root);
    [
        "1->2->4->8".to_string(),
        "1->2->4->9".to_string(),
        "1->2->5->10".to_string(),
        "1->2->5->11".to_string(),
        "1->3->6->12".to_string(),
        "1->3->6->13".to_string(),
        "1->3->7->14".to_string(),
        "1->3->7->15".to_string(),
    ]
    .into_iter()
    .for_each(|ref expected_path| {
        assert_eq!(res.contains(expected_path), true);
    });
}
