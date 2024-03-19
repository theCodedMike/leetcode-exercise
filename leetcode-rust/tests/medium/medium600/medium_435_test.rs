use leetcode_rust::leetcode::editor::cn::_435_non_overlapping_intervals::Solution;

#[test]
fn non_overlapping_intervals_1() {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 1);
}

#[test]
fn non_overlapping_intervals_2() {
    let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 2);
}

#[test]
fn non_overlapping_intervals_3() {
    let intervals = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 0);
}

#[test]
fn non_overlapping_intervals_4() {
    let intervals = vec![
        vec![-52, 31],
        vec![-73, -26],
        vec![82, 97],
        vec![-65, -11],
        vec![-62, -49],
        vec![95, 99],
        vec![58, 95],
        vec![-31, 49],
        vec![66, 98],
        vec![-63, 2],
        vec![30, 47],
        vec![-40, -26],
    ];
    assert_eq!(Solution::erase_overlap_intervals(intervals), 7);
}
