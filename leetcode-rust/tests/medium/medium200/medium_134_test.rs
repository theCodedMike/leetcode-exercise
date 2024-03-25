use leetcode_rust::leetcode::editor::cn::_134_gas_station::Solution;

#[test]
fn gas_station_1() {
    let gas = vec![1, 2, 3, 4, 5];
    let cost = vec![3, 4, 5, 1, 2];

    assert_eq!(Solution::can_complete_circuit(gas, cost), 3);
}

#[test]
fn gas_station_2() {
    let gas = vec![2, 3, 4];
    let cost = vec![3, 4, 3];

    assert_eq!(Solution::can_complete_circuit(gas, cost), -1);
}

#[test]
fn gas_station_3() {
    let gas = vec![5, 1, 2, 3, 4];
    let cost = vec![4, 4, 1, 5, 1];

    assert_eq!(Solution::can_complete_circuit(gas, cost), 4);
}
