use leetcode_exercise::binary_tree::safe::BinaryTree;
use leetcode_exercise::leetcode::editor::cn::_450_delete_node_in_a_b_s_t::Solution;
use leetcode_exercise::BuildTree;

#[test]
fn delete_node_in_a_bst_1() {
    //      5                      5
    //    /  \          3         / \
    //   3    6        ——>       4   6
    //  / \    \                /     \
    // 2   4    7              2       7

    let elems_bef = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let root = BinaryTree::build(&elems_bef);
    let res = Solution::delete_node(root, 3);

    let elems_aft = [Some(5), Some(4), Some(6), Some(2), None, None, Some(7)];
    let exp = BinaryTree::build(&elems_aft);

    assert_eq!(res, exp);
}

#[test]
fn delete_node_in_a_bst_2() {
    //      5                       5
    //    /  \          0         /  \
    //   3    6        ——>       3    6
    //  / \    \                / \    \
    // 2   4    7              2   4    7

    let elems_bef = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let root = BinaryTree::build(&elems_bef);
    let res = Solution::delete_node(root, 0);

    let elems_aft = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let exp = BinaryTree::build(&elems_aft);

    assert_eq!(res, exp);
}

#[test]
fn delete_node_in_a_bst_3() {
    let elems_bef = [];
    let root = BinaryTree::build(&elems_bef);
    let res = Solution::delete_node(root, 0);

    assert_eq!(res, None);
}

#[test]
fn delete_node_in_a_bst_4() {
    //      5                       6
    //    /  \          5         /  \
    //   3    6        ——>       3    7
    //  / \    \                / \
    // 2   4    7              2   4

    let elems_bef = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let root = BinaryTree::build(&elems_bef);
    let res = Solution::delete_node(root, 5);

    let elems_aft = [Some(6), Some(3), Some(7), Some(2), Some(4)];
    let exp = BinaryTree::build(&elems_aft);

    assert_eq!(res, exp);
}

#[test]
fn delete_node_in_a_bst_5() {
    //      0
    let elems_bef = [Some(0)];
    let root = BinaryTree::build(&elems_bef);
    let res = Solution::delete_node(root, 0);
    assert_eq!(res.is_none(), true);
}
