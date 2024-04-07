use leetcode_rust::leetcode::editor::cn::_198_house_robber::Solution;

#[test]
fn house_robber_1() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(Solution::rob(nums), 4);
}

#[test]
fn house_robber_2() {
    let nums = vec![2, 7, 9, 3, 1];
    assert_eq!(Solution::rob(nums), 12);
}

#[test]
fn house_robber_3() {
    let nums = vec![2, 1, 1, 2];
    assert_eq!(Solution::rob(nums), 4);
}
