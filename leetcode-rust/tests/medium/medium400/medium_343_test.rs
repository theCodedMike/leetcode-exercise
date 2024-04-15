use leetcode_rust::leetcode::editor::cn::_343_integer_break::Solution;

#[test]
fn integer_break_1() {
    let n = 2;
    assert_eq!(Solution::integer_break(n), 1);
}

#[test]
fn integer_break_2() {
    let n = 10;
    assert_eq!(Solution::integer_break(n), 36);
}
