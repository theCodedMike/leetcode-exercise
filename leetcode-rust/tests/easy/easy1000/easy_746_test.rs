use leetcode_rust::leetcode::editor::cn::_746_min_cost_climbing_stairs::Solution;

#[test]
fn min_cost_climbing_stairs_1() {
    let cost = vec![10, 15, 20];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
}

#[test]
fn min_cost_climbing_stairs_2() {
    let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);
}
