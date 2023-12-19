use leetcode_exercise::leetcode::editor::en::_154_find_minimum_in_rotated_sorted_array_i_i::Solution;

#[test]
fn find_minimum_in_rotated_sorted_array_ii() {
    let nums = vec![1, 3, 5];
    let min = Solution::find_min(nums);
    assert_eq!(min, 1);

    let nums = vec![2, 2, 2, 0, 1];
    let min = Solution::find_min(nums);
    assert_eq!(min, 0);

    let nums = vec![3, 1, 3, 3];
    let min = Solution::find_min(nums);
    assert_eq!(min, 1);
}
