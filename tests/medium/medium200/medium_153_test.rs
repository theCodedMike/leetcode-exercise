use leetcode_exercise::leetcode::editor::en::_153_find_minimum_in_rotated_sorted_array::Solution;

#[test]
fn find_minimum_in_rotated_sorted_array() {
    let nums = vec![3, 4, 5, 1, 2];
    let min = Solution::find_min(nums);
    assert_eq!(min, 1);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let min = Solution::find_min(nums);
    assert_eq!(min, 0);

    let nums = vec![11, 13, 15, 17];
    let min = Solution::find_min(nums);
    assert_eq!(min, 11);
}
