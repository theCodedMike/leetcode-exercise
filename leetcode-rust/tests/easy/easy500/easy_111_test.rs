use leetcode_exercise::binary_tree::safe::BinaryTree;
use leetcode_exercise::leetcode::editor::cn::_111_minimum_depth_of_binary_tree::Solution;
use leetcode_exercise::Build;

#[test]
fn minimum_depth_of_binary_tree_1() {
    //     3
    //    / \
    //   9  20
    //     /  \
    //    15   7
    let nodes = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = BinaryTree::build(&nodes);

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 2);
}

#[test]
fn minimum_depth_of_binary_tree_2() {
    //  2
    //   \
    //    3
    //     \
    //      4
    //       \
    //        5
    //         \
    //          6
    let nodes = [
        Some(2),
        None,
        Some(3),
        None,
        Some(4),
        None,
        Some(5),
        None,
        Some(6),
    ];
    let root = BinaryTree::build(&nodes);

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 5);
}

#[test]
fn minimum_depth_of_binary_tree_3() {
    //       -9
    //    /      \
    //  -3        2
    //    \      / \
    //     4    4   0
    //    /    /
    //  -6   -5
    let root = BinaryTree::build(&[
        Some(-9),
        Some(-3),
        Some(2),
        None,
        Some(4),
        Some(4),
        Some(0),
        Some(-6),
        None,
        Some(-5),
    ]);

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 3);
}

#[test]
fn minimum_depth_of_binary_tree_4() {
    //    1
    //     \
    //     -9
    //       \
    //        8
    //       / \
    //      4  -3
    //         /
    //        -3
    //        /
    //       -6
    //         \
    //         -6
    //         / \
    //       -4  -9
    //           /
    //          6
    let root = BinaryTree::build(&[
        Some(1),
        None,
        Some(-9),
        None,
        Some(8),
        Some(4),
        Some(-3),
        None,
        None,
        Some(-3),
        None,
        Some(-6),
        None,
        None,
        Some(-6),
        Some(-4),
        Some(-9),
        None,
        None,
        Some(6),
    ]);

    let min_depth = Solution::min_depth(root);
    assert_eq!(min_depth, 4);
}
