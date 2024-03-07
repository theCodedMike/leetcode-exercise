use leetcode_rust::binary_tree::safe::{BinaryTree, TreeNode};
use leetcode_rust::leetcode::editor::cn::_617_merge_two_binary_trees::Solution;
use leetcode_rust::Traverse;

#[test]
fn merge_two_binary_trees_1() {
    //     1          2                      3
    //    / \        / \                    / \
    //   3   2      1   3         =>       4   5
    //  /            \   \                / \   \
    // 5              4   7              5   4   7
    let root1 = TreeNode::new_with_children(
        1,
        TreeNode::new_with_left(3, TreeNode::new2(5)),
        TreeNode::new2(2),
    );
    let root2 = TreeNode::new_with_children(
        2,
        TreeNode::new_with_right(1, TreeNode::new2(4)),
        TreeNode::new_with_right(3, TreeNode::new2(7)),
    );
    let root = Solution::merge_trees(root1, root2);

    assert_eq!(
        BinaryTree::pre_order_recur(root.clone()),
        [3, 4, 5, 4, 5, 7]
    );
    assert_eq!(BinaryTree::in_order_recur(root.clone()), [5, 4, 4, 3, 5, 7]);
    assert_eq!(BinaryTree::post_order_recur(root), [5, 4, 4, 7, 5, 3]);
}

#[test]
fn merge_two_binary_trees_2() {
    //     1          2                       3
    //    / \        / \                    /   \
    //   3   2      1   3         =>       4     5
    //  / \   \        /                  / \   / \
    // 5   4   10     7                  5   4 7   10
    let root1 = TreeNode::new_with_children(
        1,
        TreeNode::new_with_children(3, TreeNode::new2(5), TreeNode::new2(4)),
        TreeNode::new_with_right(2, TreeNode::new2(10)),
    );
    let root2 = TreeNode::new_with_children(
        2,
        TreeNode::new2(1),
        TreeNode::new_with_left(3, TreeNode::new2(7)),
    );
    let root = Solution::merge_trees(root1, root2);

    assert_eq!(
        BinaryTree::pre_order_recur(root.clone()),
        [3, 4, 5, 4, 5, 7, 10]
    );
    assert_eq!(
        BinaryTree::in_order_recur(root.clone()),
        [5, 4, 4, 3, 7, 5, 10]
    );
    assert_eq!(BinaryTree::post_order_recur(root), [5, 4, 4, 7, 10, 5, 3]);
}

#[test]
fn merge_two_binary_trees_3() {
    //     1          1                       2
    //               /          =>          /
    //              2                      2
    let root1 = TreeNode::new2(1);
    let root2 = TreeNode::new_with_left(1, TreeNode::new2(2));
    let root = Solution::merge_trees(root1, root2);

    assert_eq!(BinaryTree::pre_order_recur(root.clone()), [2, 2]);
    assert_eq!(BinaryTree::in_order_recur(root.clone()), [2, 2]);
    assert_eq!(BinaryTree::post_order_recur(root), [2, 2]);
}

#[test]
fn merge_two_binary_trees_4() {
    //     3          4                      7
    //    / \        / \                    / \
    //   4   5      1   2         =>       5   7
    //  / \        /                      / \
    // 1   2      1                      2   2
    let root1 = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(4, TreeNode::new2(1), TreeNode::new2(2)),
        TreeNode::new2(5),
    );
    let root2 = TreeNode::new_with_children(
        4,
        TreeNode::new_with_left(1, TreeNode::new2(1)),
        TreeNode::new2(2),
    );
    let root = Solution::merge_trees(root1, root2);

    assert_eq!(BinaryTree::pre_order_recur(root.clone()), [7, 5, 2, 2, 7]);
    assert_eq!(BinaryTree::in_order_recur(root.clone()), [2, 5, 2, 7, 7]);
    assert_eq!(BinaryTree::post_order_recur(root), [2, 2, 5, 7, 7]);
}

#[test]
fn merge_two_binary_trees_5() {
    //     3          4                      7
    //    / \        / \                    / \
    //   4   5      1   2         =>       5   7
    //  / \        / \                    / \
    // 1   2      1   7                  2   9
    let root1 = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(4, TreeNode::new2(1), TreeNode::new2(2)),
        TreeNode::new2(5),
    );
    let root2 = TreeNode::new_with_children(
        4,
        TreeNode::new_with_children(1, TreeNode::new2(1), TreeNode::new2(7)),
        TreeNode::new2(2),
    );
    let root = Solution::merge_trees(root1, root2);

    assert_eq!(BinaryTree::pre_order_recur(root.clone()), [7, 5, 2, 9, 7]);
    assert_eq!(BinaryTree::in_order_recur(root.clone()), [2, 5, 9, 7, 7]);
    assert_eq!(BinaryTree::post_order_recur(root), [2, 9, 5, 7, 7]);
}
