use leetcode_exercise::leetcode::editor::en::_15_3sum::Solution;

#[test]
fn three_sum_1() {
    let nums = vec![-2, 0, 1, 1, 2];
    let res = Solution::three_sum(nums);
    assert_eq!(res, [[-2, 0, 2], [-2, 1, 1]]);
}

#[test]
fn three_sum_2() {
    let nums = vec![0, 0, 0, 0];
    let res = Solution::three_sum(nums);
    assert_eq!(res, [[0, 0, 0]]);
}

#[test]
fn three_sum_3() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let res = Solution::three_sum(nums);
    assert_eq!(res, [[-1, -1, 2], [-1, 0, 1]]);
}
