use leetcode_rust::leetcode::editor::cn::_52_n_queens_i_i::Solution;

#[test]
fn n_queens_ii_1() {
    let n = 4;
    let res = Solution::total_n_queens(n);
    assert_eq!(res, 2);
}

#[test]
fn n_queens_ii_2() {
    let n = 1;
    let res = Solution::total_n_queens(n);
    assert_eq!(res, 1);
}

#[test]
fn n_queens_ii_3() {
    let n = 5;
    let res = Solution::total_n_queens(n);
    assert_eq!(res, 10);
}
