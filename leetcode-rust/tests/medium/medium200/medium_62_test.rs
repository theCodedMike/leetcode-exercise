use leetcode_rust::leetcode::editor::cn::_62_unique_paths::Solution;

#[test]
fn unique_paths_1() {
    let m = 3;
    let n = 7;
    assert_eq!(Solution::unique_paths(m, n), 28);
}

#[test]
fn unique_paths_2() {
    let m = 3;
    let n = 2;
    assert_eq!(Solution::unique_paths(m, n), 3);
}

#[test]
fn unique_paths_3() {
    let m = 7;
    let n = 3;
    assert_eq!(Solution::unique_paths(m, n), 28);
}

#[test]
fn unique_paths_4() {
    let m = 3;
    let n = 3;
    assert_eq!(Solution::unique_paths(m, n), 6);
}
