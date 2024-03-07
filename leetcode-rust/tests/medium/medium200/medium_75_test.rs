use leetcode_rust::leetcode::editor::cn::_75_sort_colors::Solution;

#[test]
fn sort_colors() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, [0, 0, 1, 1, 2, 2]);

    let mut nums = vec![2, 0, 1];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, [0, 1, 2]);

    let mut nums = vec![0];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, [0]);

    let mut nums = vec![1];
    Solution::sort_colors(&mut nums);
    assert_eq!(nums, [1]);
}
