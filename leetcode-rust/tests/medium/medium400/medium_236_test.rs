use leetcode_rust::binary_tree::safe::TreeNode;
use leetcode_rust::leetcode::editor::cn::_236_lowest_common_ancestor_of_a_binary_tree::Solution;

#[test]
fn lowest_common_ancestor_of_a_binary_tree_1() {
    //        3
    //     /    \
    //   5       1
    //  / \     / \
    // 6   2   0   8
    //    / \
    //   7   4
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(
            5,
            TreeNode::new2(6),
            TreeNode::new_with_children(2, TreeNode::new2(7), TreeNode::new2(4)),
        ),
        TreeNode::new_with_children(1, TreeNode::new2(0), TreeNode::new2(8)),
    );
    let p = TreeNode::new2(5);
    let q = TreeNode::new2(1);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 3);
}

#[test]
fn lowest_common_ancestor_of_a_binary_tree_2() {
    //        3
    //     /    \
    //   5       1
    //  / \     / \
    // 6   2   0   8
    //    / \
    //   7   4
    let root = TreeNode::new_with_children(
        3,
        TreeNode::new_with_children(
            5,
            TreeNode::new2(6),
            TreeNode::new_with_children(2, TreeNode::new2(7), TreeNode::new2(4)),
        ),
        TreeNode::new_with_children(1, TreeNode::new2(0), TreeNode::new2(8)),
    );
    let p = TreeNode::new2(5);
    let q = TreeNode::new2(4);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 5);
}

#[test]
fn lowest_common_ancestor_of_a_binary_tree_3() {
    //    1
    //   /
    //  2
    let root = TreeNode::new_with_left(1, TreeNode::new2(2));
    let p = TreeNode::new2(1);
    let q = TreeNode::new2(2);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 1);
}

#[test]
fn lowest_common_ancestor_of_a_binary_tree_4() {
    //    1
    //   / \
    //  2   3
    //   \
    //    4
    let root = TreeNode::new_with_children(
        1,
        TreeNode::new_with_right(2, TreeNode::new2(4)),
        TreeNode::new2(3),
    );
    let p = TreeNode::new2(4);
    let q = TreeNode::new2(3);
    let res = Solution::lowest_common_ancestor(root, p, q);
    assert_eq!(res.is_some(), true);
    assert_eq!(res.unwrap().borrow().val, 1);
}
