use leetcode_exercise::binary_tree::safe::BinaryTree;
use leetcode_exercise::leetcode::editor::cn::_108_convert_sorted_array_to_binary_search_tree::Solution;
use leetcode_exercise::BuildTree;

#[test]
fn convert_sorted_array_to_binary_search_tree_1() {
    let nums = vec![-10, -3, 0, 5, 9];
    let res = Solution::sorted_array_to_bst(nums);
    //       0
    //      / \
    //    -3   9
    //   /    /
    // -10   5
    let nodes = [Some(0), Some(-3), Some(9), Some(-10), None, Some(5)];
    let exp = BinaryTree::build(&nodes);
    assert_eq!(res, exp);
}

#[test]
fn convert_sorted_array_to_binary_search_tree_2() {
    let nums = vec![1, 3];
    let res = Solution::sorted_array_to_bst(nums);
    //       3
    //      /
    //     1
    let nodes = [Some(3), Some(1)];
    let exp = BinaryTree::build(&nodes);
    assert_eq!(res, exp);
}
