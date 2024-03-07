use leetcode_rust::leetcode::editor::cn::_654_maximum_binary_tree::Solution;

#[test]
fn maximum_binary_tree_1() {
    let nums = vec![3, 2, 1, 6, 0, 5];
    let root = Solution::construct_maximum_binary_tree(nums);
    //      6
    //   /    \
    //  3      5
    //   \    /
    //    2  0
    //     \
    //      1
    println!("{:?}", root);
}

#[test]
fn maximum_binary_tree_2() {
    let nums = vec![3, 2, 1];
    let root = Solution::construct_maximum_binary_tree(nums);
    //  3
    //   \
    //    2
    //     \
    //      1
    println!("{:?}", root);
}
