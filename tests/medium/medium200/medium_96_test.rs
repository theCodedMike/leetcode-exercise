use leetcode_exercise::leetcode::editor::en::_96_unique_binary_search_trees::Solution;

#[test]
fn unique_binary_search_trees() {
    let res = Solution::num_trees(1);
    assert_eq!(res, 1);

    let res = Solution::num_trees(2);
    assert_eq!(res, 2);

    let res = Solution::num_trees(3);
    assert_eq!(res, 5);

    let res = Solution::num_trees(4);
    assert_eq!(res, 14);

    let res = Solution::num_trees(19);
    assert_eq!(res, 1767263190);
}
