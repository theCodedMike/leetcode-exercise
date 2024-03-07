use leetcode_rust::leetcode::editor::cn::_16_3_sum_closest::Solution;
#[test]
fn three_sum_closest_1() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let res = Solution::three_sum_closest(nums, target);
    assert_eq!(res, 2);

    let nums = vec![0, 0, 0];
    let target = 1;
    let res = Solution::three_sum_closest(nums, target);
    assert_eq!(res, 0);
}

#[test]
fn three_sum_closest_2() {
    let nums = vec![0, 0, 0];
    let target = 1;
    let res = Solution::three_sum_closest(nums, target);
    assert_eq!(res, 0);
}

#[test]
fn three_sum_closest_3() {
    let nums = vec![-1000, -5, -5, -5, -5, -5, -5, -1, -1, -1];
    let target = -14;
    let res = Solution::three_sum_closest(nums, target);
    assert_eq!(res, -15);
}
