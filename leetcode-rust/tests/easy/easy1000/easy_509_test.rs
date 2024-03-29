use leetcode_rust::leetcode::editor::cn::_509_fibonacci_number::Solution;

#[test]
fn fibonacci_number_1() {
    let n = 2;
    assert_eq!(Solution::fib(n), 1);
}

#[test]
fn fibonacci_number_2() {
    let n = 3;
    assert_eq!(Solution::fib(n), 2);
}

#[test]
fn fibonacci_number_3() {
    let n = 4;
    assert_eq!(Solution::fib(n), 3);
}

#[test]
fn fibonacci_number_4() {
    let n = 0;
    assert_eq!(Solution::fib(n), 0);
}

#[test]
fn fibonacci_number_5() {
    let n = 1;
    assert_eq!(Solution::fib(n), 1);
}
