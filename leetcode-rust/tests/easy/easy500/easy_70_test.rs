use leetcode_rust::leetcode::editor::cn::_70_climbing_stairs::Solution;

#[test]
fn climbing_stairs_1() {
    let n = 2;
    // 1 + 1
    // 2
    assert_eq!(Solution::climb_stairs(n), 2);
}

#[test]
fn climbing_stairs_2() {
    let n = 3;
    // 1 + 1 + 1
    // 1 + 2
    // 2 + 1
    assert_eq!(Solution::climb_stairs(n), 3);
}

#[test]
fn climbing_stairs_3() {
    let n = 45;
    // ...
    assert_eq!(Solution::climb_stairs(n), 1836311903);
}
