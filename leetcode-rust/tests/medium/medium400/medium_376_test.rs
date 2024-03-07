use leetcode_rust::leetcode::editor::cn::_376_wiggle_subsequence::Solution;

#[test]
fn wiggle_subsequence_1() {
    let nums = vec![1, 7, 4, 9, 2, 5];
    assert_eq!(Solution::wiggle_max_length(nums), 6);
}

#[test]
fn wiggle_subsequence_2() {
    let nums = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    assert_eq!(Solution::wiggle_max_length(nums), 7);
}

#[test]
fn wiggle_subsequence_3() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(Solution::wiggle_max_length(nums), 2);
}
