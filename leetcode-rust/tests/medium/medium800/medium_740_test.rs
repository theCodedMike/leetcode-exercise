use leetcode_rust::leetcode::editor::cn::_740_delete_and_earn::Solution;

#[test]
fn delete_and_earn_1() {
    let nums = vec![3, 4, 2];
    assert_eq!(Solution::delete_and_earn(nums), 6);
}

#[test]
fn delete_and_earn_2() {
    let nums = vec![2, 2, 3, 3, 3, 4];
    assert_eq!(Solution::delete_and_earn(nums), 9);
}

#[test]
fn delete_and_earn_3() {
    let nums = vec![3, 1];
    assert_eq!(Solution::delete_and_earn(nums), 4);
}
