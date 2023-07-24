use leetcode_exercise::leetcode::editor::en::_57_insert_interval::Solution;

#[test]
fn insert_interval() {
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[1, 5], [6, 9]]);

    let intervals = vec![
        vec![1, 2],
        vec![3, 5],
        vec![6, 7],
        vec![8, 10],
        vec![12, 16],
    ];
    let new_interval = vec![4, 8];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[1, 2], [3, 10], [12, 16]]);

    let intervals = vec![];
    let new_interval = vec![5, 7];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[5, 7]]);

    let intervals = vec![vec![1, 5]];
    let new_interval = vec![6, 8];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[1, 5], [6, 8]]);

    let intervals = vec![vec![1, 5]];
    let new_interval = vec![0, 3];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[0, 5]]);

    let intervals = vec![vec![3, 5], vec![12, 15]];
    let new_interval = vec![6, 6];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[3, 5], [6, 6], [12, 15]]);

    let intervals = vec![vec![2, 5], vec![6, 7], vec![8, 9]];
    let new_interval = vec![0, 1];
    let res = Solution::insert(intervals, new_interval);
    assert_eq!(res, [[0, 1], [2, 5], [6, 7], [8, 9]]);
}
