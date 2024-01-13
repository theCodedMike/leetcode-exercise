use leetcode_exercise::leetcode::editor::cn::_80_remove_duplicates_from_sorted_array_i_i::Solution;

#[test]
fn remove_duplicates_from_sorted_array_ii() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    Solution::remove_duplicates(&mut nums);
    assert_eq!(nums, [1, 1, 2, 2, 3, 3]);

    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    Solution::remove_duplicates(&mut nums);
    assert_eq!(nums, [0, 0, 1, 1, 2, 3, 3, 3, 3]);

    let mut nums = vec![1, 1, 1, 1, 2, 2, 3];
    Solution::remove_duplicates(&mut nums);
    assert_eq!(nums, [1, 1, 2, 2, 3, 2, 3]);
}
