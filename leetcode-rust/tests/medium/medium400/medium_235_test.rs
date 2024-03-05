use leetcode_exercise::binary_tree::safe::TreeNode;
use leetcode_exercise::leetcode::editor::cn::_235_lowest_common_ancestor_of_a_binary_search_tree::Solution;

#[test]
fn lowest_common_ancestor_of_a_binary_search_tree_1() {
    //         6
    //      /     \
    //     2       8
    //    / \     / \
    //   0   4   7   9
    //      / \
    //     3   5
    let root = TreeNode::new_with_children(
        6,
        TreeNode::new_with_children(
            2,
            TreeNode::new2(0),
            TreeNode::new_with_children(4, TreeNode::new2(3), TreeNode::new2(5)),
        ),
        TreeNode::new_with_children(8, TreeNode::new2(7), TreeNode::new2(9)),
    );
    let p = TreeNode::new2(2);
    let q = TreeNode::new2(8);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 6);
}

#[test]
fn lowest_common_ancestor_of_a_binary_search_tree_2() {
    //         6
    //      /     \
    //     2       8
    //    / \     / \
    //   0   4   7   9
    //      / \
    //     3   5
    let root = TreeNode::new_with_children(
        6,
        TreeNode::new_with_children(
            2,
            TreeNode::new2(0),
            TreeNode::new_with_children(4, TreeNode::new2(3), TreeNode::new2(5)),
        ),
        TreeNode::new_with_children(8, TreeNode::new2(7), TreeNode::new2(9)),
    );
    let p = TreeNode::new2(2);
    let q = TreeNode::new2(4);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 2);
}

#[test]
fn lowest_common_ancestor_of_a_binary_search_tree_3() {
    //    2
    //   /
    //  1
    let root = TreeNode::new_with_left(2, TreeNode::new2(1));
    let p = TreeNode::new2(2);
    let q = TreeNode::new2(1);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 2);
}
