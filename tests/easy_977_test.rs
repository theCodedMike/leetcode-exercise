use leetcode_exercise::leetcode::editor::en::_977_squares_of_a_sorted_array::Solution;

#[test]
fn squares_of_a_sorted_array() {
    let nums = vec![-4, -1, 0, 3, 10];
    let res = Solution::sorted_squares(nums);
    assert_eq!(res, [0, 1, 9, 16, 100]);

    let nums = vec![-7, -3, 2, 3, 11];
    let res = Solution::sorted_squares(nums);
    assert_eq!(res, [4, 9, 9, 49, 121]);

    let nums = vec![-10000, -9999, -7, -5, 0, 0, 10000];
    let res = Solution::sorted_squares(nums);
    assert_eq!(res, [0, 0, 25, 49, 99980001, 100000000, 100000000]);
}
