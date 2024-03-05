use leetcode_exercise::binary_tree::safe::BinaryTree;
use leetcode_exercise::leetcode::editor::cn::_100_same_tree::Solution;
use leetcode_exercise::Build;

#[test]
fn same_tree_1() {
    // p:    1           q:    1
    //      / \               / \
    //     2   3             2   3
    let p = BinaryTree::build(&[Some(1), Some(2), Some(3)]);
    let q = BinaryTree::build(&[Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::is_same_tree(p, q), true);
}

#[test]
fn same_tree_2() {
    // p:    1           q:    1
    //      /                   \
    //     2                     2
    let p = BinaryTree::build(&[Some(1), Some(2)]);
    let q = BinaryTree::build(&[Some(1), None, Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}

#[test]
fn same_tree_3() {
    // p:    1           q:    1
    //      / \               / \
    //     2   1             1   2
    let p = BinaryTree::build(&[Some(1), Some(2), Some(1)]);
    let q = BinaryTree::build(&[Some(1), Some(1), Some(2)]);
    assert_eq!(Solution::is_same_tree(p, q), false);
}
