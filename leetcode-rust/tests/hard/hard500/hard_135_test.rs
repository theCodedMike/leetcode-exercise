use leetcode_rust::leetcode::editor::cn::_135_candy::Solution;

#[test]
fn candy_1() {
    let ratings = vec![1, 0, 2];
    assert_eq!(Solution::candy(ratings), 5);
}

#[test]
fn candy_2() {
    let ratings = vec![1, 2, 2];
    assert_eq!(Solution::candy(ratings), 4);
}

#[test]
fn candy_3() {
    let ratings = vec![1, 2, 87, 87, 87, 2, 1];
    assert_eq!(Solution::candy(ratings), 13);
}
