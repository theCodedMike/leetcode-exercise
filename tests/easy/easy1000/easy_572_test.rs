use leetcode_exercise::binary_tree::safe::{BinaryTree, TreeNode};
use leetcode_exercise::leetcode::editor::cn::_572_subtree_of_another_tree::Solution;
use leetcode_exercise::Build;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn subtree_of_another_tree_1() {
    // root    3           subRoot    4
    //        / \                    / \
    //       4   5                  1   2
    //      / \
    //     1   2
    let root = BinaryTree::build(&[Some(3), Some(4), Some(5), Some(1), Some(2)]);
    let sub_root = BinaryTree::build(&[Some(4), Some(1), Some(2)]);

    assert_eq!(Solution::is_subtree(root, sub_root), true);
}

#[test]
fn subtree_of_another_tree_2() {
    // root    3           subRoot    4
    //        / \                    / \
    //       4   5                  1   2
    //      / \
    //     1   2
    //        /
    //       0
    let root = BinaryTree::build(&[
        Some(3),
        Some(4),
        Some(5),
        Some(1),
        Some(2),
        None,
        None,
        None,
        None,
        Some(0),
    ]);
    let sub_root = BinaryTree::build(&[Some(4), Some(1), Some(2)]);

    assert_eq!(Solution::is_subtree(root, sub_root), false);
}

#[test]
fn subtree_of_another_tree_3() {
    // root    1           subRoot    1
    //        /
    //       1
    let root = BinaryTree::build(&[Some(1), Some(1)]);
    let sub_root = BinaryTree::build(&[Some(1)]);

    assert_eq!(Solution::is_subtree(root, sub_root), true);
}

#[test]
fn subtree_of_another_tree_4() {
    // root    3           subRoot    3
    //        / \                    / \
    //       4   5                  1   2
    //      /   /
    //     1   2
    let root = BinaryTree::build(&[Some(3), Some(4), Some(5), Some(1), None, Some(2)]);
    let sub_root = BinaryTree::build(&[Some(3), Some(1), Some(2)]);

    assert_eq!(Solution::is_subtree(root, sub_root), false);
}

#[test]
fn subtree_of_another_tree_5() {
    // root    1           subRoot    1
    //          \                      \
    //           1                      1
    //            \                      \
    //             1                      1
    //              \                      \
    //               1                      1
    //                \                      \
    //                 1                      1
    //                  \                      \
    //                   1                      1
    //                    \                    /
    //                     1                  2
    //                      \
    //                       1
    //                        \
    //                         1
    //                          \
    //                           1
    //                            \
    //                             1
    //                            /
    //                           2
    let root = BinaryTree::build(&[
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        Some(2),
    ]);
    let sub_root = BinaryTree::build(&[
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        None,
        Some(1),
        Some(2),
    ]);

    assert_eq!(Solution::is_subtree(root, sub_root), true);
}

#[test]
fn subtree_of_another_tree_6() {
    let vals = vec![
        "73", "74", "72", "73", "73", "71", "73", "72", "72", "74", "72", "null", "72", "74", "72",
        "73", "71", "71", "71", "73", "null", "71", "73", "73", "73", "75", "75", "71", "73", "74",
        "74", "70", "70", "70", "72", "70", "72", "74", "74", "72", "72", "72", "null", "72",
        "null", "null", "null", "null", "null", "74", "76", "70", "72", "null", "72", "75", "75",
        "null", "null", "null", "71", "null", "null", "71", "71", "71", "73", "71", "null", "71",
        "71", "null", "75", "null", "null", "71", "73", "71", "73", "null", "73", "null", "null",
        "null", "75", "null", "77", "null", "71", "null", "null", "null", "73", "74", "null", "74",
        "null", "72", "null", "72", "70", "72", "null", "null", "null", "72", "74", "70", "null",
        "72", "null", "null", "72", "null", "null", "72", "null", "74", "74", "70", "null", "null",
        "null", "74", "null", "74", "null", "76", "null", "null", "null", "72", "74", "75", "75",
        "73", "73", "71", "73", "73", "null", "69", "null", "71", "73", "71", "71", "73", "null",
        "null", "null", "null", "null", "73", "71", "71", "73", "null", "null", "73", "73", "69",
        "69", "75", "73", "null", "null", "null", "null", "null", "null", "null", "null", "null",
        "null", "null", "null", "null", "null", "null", "null", "null", "null", "72", "null", "72",
        "null", "null", "70", "70", "null", "null", "74", "70", "72", "70", "null", "null", "null",
        "72", "null", "70", "70", "null", "null", "72", "74", "null", "null", "null", "null", "68",
        "70", "null", "70", "null", "null", "null", "74", "null", "null", "null", "null", "null",
        "null", "null", "null", "null", "null", "null", "null", "71", "null", "null", "null", "73",
        "73", "null", "null", "null", "71", "73", "null", "75", "75", "67", "null", "null", "71",
        "71", "null", "null", "null", "null", "72", "null", "72", "null", "null", "null", "null",
        "74", "null", "null", "null", "76", "74", "null", "null", "null", "72", "null", "null",
        "null", "null", "null", "null", "null", "null", "77", "77", "null", "75", "null", "null",
        "76", "76", "null", "null", "null", "null", "75", "77", "null", "null", "null", "74",
    ];
    let root = convert(&vals);

    // [72,71,71,70,72,70,null,null,null,71,null,null,null,null,72]
    //              72
    //           /      \
    //         71        71
    //        /  \      /
    //      70    72   70
    //           /
    //          71
    //            \
    //             72
    let vals = vec![
        "72", "71", "71", "70", "72", "70", "null", "null", "null", "71", "null", "null", "null",
        "null", "72",
    ];
    let sub_root = convert(&vals);

    assert_eq!(Solution::is_subtree(root, sub_root), true);
}

fn convert(vals: &[&str]) -> Option<Rc<RefCell<TreeNode>>> {
    let std_vals = vals
        .into_iter()
        .map(|&v| {
            if v == "null" {
                None
            } else {
                v.parse::<i32>().ok()
            }
        })
        .collect::<Vec<Option<i32>>>();

    BinaryTree::build(&std_vals)
}
