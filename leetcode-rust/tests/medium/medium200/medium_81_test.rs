use leetcode_rust::leetcode::editor::cn::_81_search_in_rotated_sorted_array_i_i::Solution;

#[test]
fn search_in_rotated_sorted_array_ii() {
    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 0;
    let res = Solution::search(nums, target);
    assert_eq!(res, true);

    let nums = vec![2, 5, 6, 0, 0, 1, 2];
    let target = 3;
    let res = Solution::search(nums, target);
    assert_eq!(res, false);

    let nums = vec![2];
    let target = 2;
    let res = Solution::search(nums, target);
    assert_eq!(res, true);

    let nums = vec![2];
    let target = 3;
    let res = Solution::search(nums, target);
    assert_eq!(res, false);

    let nums = vec![1, 3];
    let target = 1;
    let res = Solution::search(nums, target);
    assert_eq!(res, true);

    let nums = vec![3, 1];
    let target = 1;
    let res = Solution::search(nums, target);
    assert_eq!(res, true);
}
