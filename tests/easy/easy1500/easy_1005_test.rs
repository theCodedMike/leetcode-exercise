use leetcode_exercise::leetcode::editor::cn::_1005_maximize_sum_of_array_after_k_negations::Solution;

#[test]
fn maximize_sum_of_array_after_k_negations_1() {
    let nums = vec![4, 2, 3];
    let k = 1;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 4 + -2 + 3 = 5
    assert_eq!(res, 5);
}

#[test]
fn maximize_sum_of_array_after_k_negations_2() {
    let nums = vec![3, -1, 0, 2];
    let k = 3;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 3 + 1 + 0 + 2 = 6
    assert_eq!(res, 6);
}

#[test]
fn maximize_sum_of_array_after_k_negations_3() {
    let nums = vec![2, -3, -1, 5, -4];
    let k = 2;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 2 + 3 + -1 + 5 + 4 = 13
    assert_eq!(res, 13);
}

#[test]
fn maximize_sum_of_array_after_k_negations_4() {
    let nums = vec![-2, 9, 9, 8, 4];
    let k = 5;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 2 + 9 + 9 + 8 + 4 = 32
    assert_eq!(res, 32);
}

#[test]
fn maximize_sum_of_array_after_k_negations_5() {
    let nums = vec![-2, 5, 0, 2, -2];
    let k = 3;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 2 + 5 + 0 + 2 + 2 = 11
    assert_eq!(res, 11);
}

#[test]
fn maximize_sum_of_array_after_k_negations_6() {
    let nums = vec![1, 3, 2, 6, 7, 9];
    let k = 3;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // -1 + 3 + 2 + 6 + 7 + 9 = 26
    assert_eq!(res, 26);
}

#[test]
fn maximize_sum_of_array_after_k_negations_7() {
    let nums = vec![-8, 3, -5, -3, -5, -2];
    let k = 6;
    let res = Solution::largest_sum_after_k_negations(nums, k);
    // 8 + 3 + 5 + 3 + 5 + 2 = 22
    assert_eq!(res, 22);
}
