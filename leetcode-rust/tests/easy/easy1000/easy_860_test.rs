use leetcode_rust::leetcode::editor::cn::_860_lemonade_change::Solution;

#[test]
fn lemonade_change_test_1() {
    let bills = vec![5, 5, 5, 10, 20];
    assert_eq!(Solution::lemonade_change(bills), true);
}

#[test]
fn lemonade_change_test_2() {
    let bills = vec![5, 5, 10, 10, 20];
    assert_eq!(Solution::lemonade_change(bills), false);
}

#[test]
fn lemonade_change_test_3() {
    let bills = vec![
        5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
    ];
    assert_eq!(Solution::lemonade_change(bills), true);
}
