use leetcode_exercise::leetcode::editor::cn::_209_minimum_size_subarray_sum::Solution;

#[test]
fn minimum_size_subarray_sum() {
    let target = 7;
    let nums = vec![2, 3, 1, 2, 4, 3];
    let res = Solution::min_sub_array_len(target, nums);
    assert_eq!(res, 2);

    let target = 4;
    let nums = vec![1, 4, 4];
    let res = Solution::min_sub_array_len(target, nums);
    assert_eq!(res, 1);

    let target = 11;
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let res = Solution::min_sub_array_len(target, nums);
    assert_eq!(res, 0);

    let target = 11;
    let nums = vec![1, 2, 3, 4, 5];
    let res = Solution::min_sub_array_len(target, nums);
    assert_eq!(res, 3);
}
