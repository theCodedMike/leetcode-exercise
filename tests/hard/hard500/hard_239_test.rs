use leetcode_exercise::leetcode::editor::cn::_239_sliding_window_maximum::Solution;

#[test]
fn sliding_window_maximum_test_1() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k = 3;
    let res = Solution::max_sliding_window(nums, k);
    assert_eq!(res, [3, 3, 5, 5, 6, 7]);
}

#[test]
fn sliding_window_maximum_test_2() {
    let nums = vec![1];
    let k = 1;
    let res = Solution::max_sliding_window(nums, k);
    assert_eq!(res, [1]);
}

#[test]
fn sliding_window_maximum_test_3() {
    let nums = vec![1, -1];
    let k = 1;
    let res = Solution::max_sliding_window(nums, k);
    assert_eq!(res, [1, -1]);
}
