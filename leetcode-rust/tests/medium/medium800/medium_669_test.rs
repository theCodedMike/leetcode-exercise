use leetcode_rust::binary_tree::safe::BinaryTree;
use leetcode_rust::leetcode::editor::cn::_669_trim_a_binary_search_tree::Solution;
use leetcode_rust::Build;

#[test]
fn trim_a_binary_search_tree_1() {
    //    1       [1,2]        1
    //   / \       =>           \
    //  0   2                    2
    let nodes_bfr = vec![Some(1), Some(0), Some(2)];
    let root = BinaryTree::build(&nodes_bfr);

    let res = Solution::trim_bst(root, 1, 2);

    let nodes_aft = vec![Some(1), None, Some(2)];
    let exp = BinaryTree::build(&nodes_aft);

    assert_eq!(res, exp)
}

#[test]
fn trim_a_binary_search_tree_2() {
    //    3       [1,3]       3
    //   / \       =>        /
    //  0   4               2
    //   \                 /
    //    2               1
    //   /
    //  1
    let nodes_bfr = [
        Some(3),
        Some(0),
        Some(4),
        None,
        Some(2),
        None,
        None,
        Some(1),
    ];
    let root = BinaryTree::build(&nodes_bfr);

    let res = Solution::trim_bst(root, 1, 3);

    let nodes_aft = [Some(3), Some(2), None, Some(1)];
    let exp = BinaryTree::build(&nodes_aft);

    assert_eq!(res, exp)
}

#[test]
fn trim_a_binary_search_tree_3() {
    //    3       [1,2]
    //   / \       =>
    //  0   4              2
    //   \                /
    //    2              1
    //   /
    //  1
    let nodes_bfr = [
        Some(3),
        Some(0),
        Some(4),
        None,
        Some(2),
        None,
        None,
        Some(1),
    ];
    let root = BinaryTree::build(&nodes_bfr);

    let res = Solution::trim_bst(root, 1, 2);

    let nodes_aft = [Some(2), Some(1)];
    let exp = BinaryTree::build(&nodes_aft);

    assert_eq!(res, exp)
}
