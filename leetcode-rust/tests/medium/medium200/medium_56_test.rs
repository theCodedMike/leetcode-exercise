use leetcode_rust::leetcode::editor::cn::_56_merge_intervals::Solution;

#[test]
fn merge_intervals_1() {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let res = Solution::merge(intervals);
    assert_eq!(res, [[1, 6], [8, 10], [15, 18]]);
}

#[test]
fn merge_intervals_2() {
    let intervals = vec![vec![1, 4], vec![4, 5]];
    let res = Solution::merge(intervals);
    assert_eq!(res, [[1, 5]]);
}

#[test]
fn merge_intervals_3() {
    let intervals = vec![vec![1, 4], vec![0, 4]];
    let res = Solution::merge(intervals);
    assert_eq!(res, [[0, 4]]);
}

#[test]
fn merge_intervals_4() {
    let intervals = vec![vec![1, 4], vec![2, 3]];
    let res = Solution::merge(intervals);
    assert_eq!(res, [[1, 4]]);
}
