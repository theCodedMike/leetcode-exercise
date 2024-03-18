use leetcode_rust::leetcode::editor::cn::_452_minimum_number_of_arrows_to_burst_balloons::Solution;

#[test]
fn minimum_number_of_arrows_to_burst_balloons_1() {
    let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    assert_eq!(Solution::find_min_arrow_shots(points), 2);
}

#[test]
fn minimum_number_of_arrows_to_burst_balloons_2() {
    let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
    assert_eq!(Solution::find_min_arrow_shots(points), 4);
}

#[test]
fn minimum_number_of_arrows_to_burst_balloons_3() {
    let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
    assert_eq!(Solution::find_min_arrow_shots(points), 2);
}
