use leetcode_exercise::leetcode::editor::cn::_29_divide_two_integers::Solution;

#[test]
fn divide_two_integers() {
    let res = Solution::divide(10, 3);
    assert_eq!(res, 3);

    let res = Solution::divide(7, -3);
    assert_eq!(res, -2);
}
