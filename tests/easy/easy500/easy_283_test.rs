use leetcode_exercise::leetcode::editor::en::_283_move_zeroes::Solution;

#[test]
fn move_zeroes() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [1, 3, 12, 0, 0]);

    let mut nums = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [0]);

    let mut nums = vec![1];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [1]);

    let mut nums = vec![2, 1];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, [2, 1]);
}
