use leetcode_exercise::leetcode::editor::cn::_41_first_missing_positive::Solution;

#[test]
fn first_missing_positive() {
    let nums = vec![1, 2, 0];
    let res = Solution::first_missing_positive(nums);
    assert_eq!(res, 3);

    let nums = vec![3, 4, -1, 1];
    let res = Solution::first_missing_positive(nums);
    assert_eq!(res, 2);

    let nums = vec![7, 8, 9, 11, 12];
    let res = Solution::first_missing_positive(nums);
    assert_eq!(res, 1);
}
