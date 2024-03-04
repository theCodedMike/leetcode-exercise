use leetcode_exercise::leetcode::editor::cn::_738_monotone_increasing_digits::Solution;

#[test]
fn monotone_increasing_digits_1() {
    let n = 10;
    assert_eq!(Solution::monotone_increasing_digits(n), 9);
}

#[test]
fn monotone_increasing_digits_2() {
    let n = 1234;
    assert_eq!(Solution::monotone_increasing_digits(n), 1234);
}

#[test]
fn monotone_increasing_digits_3() {
    let n = 332;
    assert_eq!(Solution::monotone_increasing_digits(n), 299);
}

#[test]
fn monotone_increasing_digits_4() {
    let n = 6;
    assert_eq!(Solution::monotone_increasing_digits(n), 6);
}

#[test]
fn monotone_increasing_digits_5() {
    let n = 528357107;
    assert_eq!(Solution::monotone_increasing_digits(n), 499999999);
}

#[test]
fn monotone_increasing_digits_6() {
    let n = 654044082;
    assert_eq!(Solution::monotone_increasing_digits(n), 599999999);
}

#[test]
fn monotone_increasing_digits_7() {
    let n = 120;
    assert_eq!(Solution::monotone_increasing_digits(n), 119);
}
