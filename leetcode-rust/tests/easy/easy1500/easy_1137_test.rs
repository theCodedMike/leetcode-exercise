use leetcode_rust::leetcode::editor::cn::_1137_n_th_tribonacci_number::Solution;

#[test]
fn n_th_tribonacci_number_1() {
    let n = 0;
    assert_eq!(Solution::tribonacci(n), 0);
}

#[test]
fn n_th_tribonacci_number_2() {
    let n = 1;
    assert_eq!(Solution::tribonacci(n), 1);
}

#[test]
fn n_th_tribonacci_number_3() {
    let n = 2;
    assert_eq!(Solution::tribonacci(n), 1);
}

#[test]
fn n_th_tribonacci_number_4() {
    let n = 3;
    assert_eq!(Solution::tribonacci(n), 2);
}

#[test]
fn n_th_tribonacci_number_5() {
    let n = 4;
    assert_eq!(Solution::tribonacci(n), 4);
}

#[test]
fn n_th_tribonacci_number_6() {
    let n = 25;
    assert_eq!(Solution::tribonacci(n), 1389537);
}
