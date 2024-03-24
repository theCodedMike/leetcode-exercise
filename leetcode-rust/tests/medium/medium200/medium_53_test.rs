use leetcode_rust::leetcode::editor::cn::_53_maximum_subarray::Solution;

#[test]
fn maximum_subarray_1() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(Solution::max_sub_array(nums), 6);
}

#[test]
fn maximum_subarray_2() {
    let nums = vec![1];
    assert_eq!(Solution::max_sub_array(nums), 1);
}

#[test]
fn maximum_subarray_3() {
    let nums = vec![5, 4, -1, 7, 8];
    assert_eq!(Solution::max_sub_array(nums), 23);
}

#[test]
fn maximum_subarray_4() {
    let nums = vec![-1];
    assert_eq!(Solution::max_sub_array(nums), -1);
}

#[test]
fn maximum_subarray_5() {
    let nums = vec![1, -1, -2];
    assert_eq!(Solution::max_sub_array(nums), 1);
}

#[test]
fn maximum_subarray_6() {
    let nums = vec![3, -2, -1];
    assert_eq!(Solution::max_sub_array(nums), 3);
}
