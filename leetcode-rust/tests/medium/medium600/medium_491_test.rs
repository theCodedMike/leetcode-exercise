use leetcode_rust::leetcode::editor::cn::_491_non_decreasing_subsequences::Solution;
use std::collections::HashSet;

#[test]
fn non_decreasing_subsequences_1() {
    let nums = vec![4, 6, 7, 7];
    let res = Solution::find_subsequences(nums);

    assert_eq!(res.len(), 8);
    let set = HashSet::from([
        vec![4, 6],
        vec![4, 6, 7],
        vec![4, 6, 7, 7],
        vec![4, 7],
        vec![4, 7, 7],
        vec![6, 7],
        vec![6, 7, 7],
        vec![7, 7],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn non_decreasing_subsequences_2() {
    let nums = vec![4, 4, 3, 2, 1];
    let res = Solution::find_subsequences(nums);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec![4, 4]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn non_decreasing_subsequences_3() {
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 1, 1, 1, 1];
    let res = Solution::find_subsequences(nums);

    assert_eq!(res.len(), 1018);
    let set = HashSet::from([
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5, 6],
        vec![1, 2, 3, 4, 5, 6, 7],
        vec![1, 2, 3, 4, 5, 6, 7, 8],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 8, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 9],
        vec![1, 2, 3, 4, 5, 6, 7, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 7, 10],
        vec![1, 2, 3, 4, 5, 6, 8],
        vec![1, 2, 3, 4, 5, 6, 8, 9],
        vec![1, 2, 3, 4, 5, 6, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 8, 10],
        vec![1, 2, 3, 4, 5, 6, 9],
        vec![1, 2, 3, 4, 5, 6, 9, 10],
        vec![1, 2, 3, 4, 5, 6, 10],
        vec![1, 2, 3, 4, 5, 7],
        vec![1, 2, 3, 4, 5, 7, 8],
        vec![1, 2, 3, 4, 5, 7, 8, 9],
        vec![1, 2, 3, 4, 5, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 7, 8, 10],
        vec![1, 2, 3, 4, 5, 7, 9],
        vec![1, 2, 3, 4, 5, 7, 9, 10],
        vec![1, 2, 3, 4, 5, 7, 10],
        vec![1, 2, 3, 4, 5, 8],
        vec![1, 2, 3, 4, 5, 8, 9],
        vec![1, 2, 3, 4, 5, 8, 9, 10],
        vec![1, 2, 3, 4, 5, 8, 10],
        vec![1, 2, 3, 4, 5, 9],
        vec![1, 2, 3, 4, 5, 9, 10],
        vec![1, 2, 3, 4, 5, 10],
        vec![1, 2, 3, 4, 6],
        vec![1, 2, 3, 4, 6, 7],
        vec![1, 2, 3, 4, 6, 7, 8],
        vec![1, 2, 3, 4, 6, 7, 8, 9],
        vec![1, 2, 3, 4, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 6, 7, 8, 10],
        vec![1, 2, 3, 4, 6, 7, 9],
        vec![1, 2, 3, 4, 6, 7, 9, 10],
        vec![1, 2, 3, 4, 6, 7, 10],
        vec![1, 2, 3, 4, 6, 8],
        vec![1, 2, 3, 4, 6, 8, 9],
        vec![1, 2, 3, 4, 6, 8, 9, 10],
        vec![1, 2, 3, 4, 6, 8, 10],
        vec![1, 2, 3, 4, 6, 9],
        vec![1, 2, 3, 4, 6, 9, 10],
        vec![1, 2, 3, 4, 6, 10],
        vec![1, 2, 3, 4, 7],
        vec![1, 2, 3, 4, 7, 8],
        vec![1, 2, 3, 4, 7, 8, 9],
        vec![1, 2, 3, 4, 7, 8, 9, 10],
        vec![1, 2, 3, 4, 7, 8, 10],
        vec![1, 2, 3, 4, 7, 9],
        vec![1, 2, 3, 4, 7, 9, 10],
        vec![1, 2, 3, 4, 7, 10],
        vec![1, 2, 3, 4, 8],
        vec![1, 2, 3, 4, 8, 9],
        vec![1, 2, 3, 4, 8, 9, 10],
        vec![1, 2, 3, 4, 8, 10],
        vec![1, 2, 3, 4, 9],
        vec![1, 2, 3, 4, 9, 10],
        vec![1, 2, 3, 4, 10],
        vec![1, 2, 3, 5],
        vec![1, 2, 3, 5, 6],
        vec![1, 2, 3, 5, 6, 7],
        vec![1, 2, 3, 5, 6, 7, 8],
        vec![1, 2, 3, 5, 6, 7, 8, 9],
        vec![1, 2, 3, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 5, 6, 7, 8, 10],
        vec![1, 2, 3, 5, 6, 7, 9],
        vec![1, 2, 3, 5, 6, 7, 9, 10],
        vec![1, 2, 3, 5, 6, 7, 10],
        vec![1, 2, 3, 5, 6, 8],
        vec![1, 2, 3, 5, 6, 8, 9],
        vec![1, 2, 3, 5, 6, 8, 9, 10],
        vec![1, 2, 3, 5, 6, 8, 10],
        vec![1, 2, 3, 5, 6, 9],
        vec![1, 2, 3, 5, 6, 9, 10],
        vec![1, 2, 3, 5, 6, 10],
        vec![1, 2, 3, 5, 7],
        vec![1, 2, 3, 5, 7, 8],
        vec![1, 2, 3, 5, 7, 8, 9],
        vec![1, 2, 3, 5, 7, 8, 9, 10],
        vec![1, 2, 3, 5, 7, 8, 10],
        vec![1, 2, 3, 5, 7, 9],
        vec![1, 2, 3, 5, 7, 9, 10],
        vec![1, 2, 3, 5, 7, 10],
        vec![1, 2, 3, 5, 8],
        vec![1, 2, 3, 5, 8, 9],
        vec![1, 2, 3, 5, 8, 9, 10],
        vec![1, 2, 3, 5, 8, 10],
        vec![1, 2, 3, 5, 9],
        vec![1, 2, 3, 5, 9, 10],
        vec![1, 2, 3, 5, 10],
        vec![1, 2, 3, 6],
        vec![1, 2, 3, 6, 7],
        vec![1, 2, 3, 6, 7, 8],
        vec![1, 2, 3, 6, 7, 8, 9],
        vec![1, 2, 3, 6, 7, 8, 9, 10],
        vec![1, 2, 3, 6, 7, 8, 10],
        vec![1, 2, 3, 6, 7, 9],
        vec![1, 2, 3, 6, 7, 9, 10],
        vec![1, 2, 3, 6, 7, 10],
        vec![1, 2, 3, 6, 8],
        vec![1, 2, 3, 6, 8, 9],
        vec![1, 2, 3, 6, 8, 9, 10],
        vec![1, 2, 3, 6, 8, 10],
        vec![1, 2, 3, 6, 9],
        vec![1, 2, 3, 6, 9, 10],
        vec![1, 2, 3, 6, 10],
        vec![1, 2, 3, 7],
        vec![1, 2, 3, 7, 8],
        vec![1, 2, 3, 7, 8, 9],
        vec![1, 2, 3, 7, 8, 9, 10],
        vec![1, 2, 3, 7, 8, 10],
        vec![1, 2, 3, 7, 9],
        vec![1, 2, 3, 7, 9, 10],
        vec![1, 2, 3, 7, 10],
        vec![1, 2, 3, 8],
        vec![1, 2, 3, 8, 9],
        vec![1, 2, 3, 8, 9, 10],
        vec![1, 2, 3, 8, 10],
        vec![1, 2, 3, 9],
        vec![1, 2, 3, 9, 10],
        vec![1, 2, 3, 10],
        vec![1, 2, 4],
        vec![1, 2, 4, 5],
        vec![1, 2, 4, 5, 6],
        vec![1, 2, 4, 5, 6, 7],
        vec![1, 2, 4, 5, 6, 7, 8],
        vec![1, 2, 4, 5, 6, 7, 8, 9],
        vec![1, 2, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 4, 5, 6, 7, 8, 10],
        vec![1, 2, 4, 5, 6, 7, 9],
        vec![1, 2, 4, 5, 6, 7, 9, 10],
        vec![1, 2, 4, 5, 6, 7, 10],
        vec![1, 2, 4, 5, 6, 8],
        vec![1, 2, 4, 5, 6, 8, 9],
        vec![1, 2, 4, 5, 6, 8, 9, 10],
        vec![1, 2, 4, 5, 6, 8, 10],
        vec![1, 2, 4, 5, 6, 9],
        vec![1, 2, 4, 5, 6, 9, 10],
        vec![1, 2, 4, 5, 6, 10],
        vec![1, 2, 4, 5, 7],
        vec![1, 2, 4, 5, 7, 8],
        vec![1, 2, 4, 5, 7, 8, 9],
        vec![1, 2, 4, 5, 7, 8, 9, 10],
        vec![1, 2, 4, 5, 7, 8, 10],
        vec![1, 2, 4, 5, 7, 9],
        vec![1, 2, 4, 5, 7, 9, 10],
        vec![1, 2, 4, 5, 7, 10],
        vec![1, 2, 4, 5, 8],
        vec![1, 2, 4, 5, 8, 9],
        vec![1, 2, 4, 5, 8, 9, 10],
        vec![1, 2, 4, 5, 8, 10],
        vec![1, 2, 4, 5, 9],
        vec![1, 2, 4, 5, 9, 10],
        vec![1, 2, 4, 5, 10],
        vec![1, 2, 4, 6],
        vec![1, 2, 4, 6, 7],
        vec![1, 2, 4, 6, 7, 8],
        vec![1, 2, 4, 6, 7, 8, 9],
        vec![1, 2, 4, 6, 7, 8, 9, 10],
        vec![1, 2, 4, 6, 7, 8, 10],
        vec![1, 2, 4, 6, 7, 9],
        vec![1, 2, 4, 6, 7, 9, 10],
        vec![1, 2, 4, 6, 7, 10],
        vec![1, 2, 4, 6, 8],
        vec![1, 2, 4, 6, 8, 9],
        vec![1, 2, 4, 6, 8, 9, 10],
        vec![1, 2, 4, 6, 8, 10],
        vec![1, 2, 4, 6, 9],
        vec![1, 2, 4, 6, 9, 10],
        vec![1, 2, 4, 6, 10],
        vec![1, 2, 4, 7],
        vec![1, 2, 4, 7, 8],
        vec![1, 2, 4, 7, 8, 9],
        vec![1, 2, 4, 7, 8, 9, 10],
        vec![1, 2, 4, 7, 8, 10],
        vec![1, 2, 4, 7, 9],
        vec![1, 2, 4, 7, 9, 10],
        vec![1, 2, 4, 7, 10],
        vec![1, 2, 4, 8],
        vec![1, 2, 4, 8, 9],
        vec![1, 2, 4, 8, 9, 10],
        vec![1, 2, 4, 8, 10],
        vec![1, 2, 4, 9],
        vec![1, 2, 4, 9, 10],
        vec![1, 2, 4, 10],
        vec![1, 2, 5],
        vec![1, 2, 5, 6],
        vec![1, 2, 5, 6, 7],
        vec![1, 2, 5, 6, 7, 8],
        vec![1, 2, 5, 6, 7, 8, 9],
        vec![1, 2, 5, 6, 7, 8, 9, 10],
        vec![1, 2, 5, 6, 7, 8, 10],
        vec![1, 2, 5, 6, 7, 9],
        vec![1, 2, 5, 6, 7, 9, 10],
        vec![1, 2, 5, 6, 7, 10],
        vec![1, 2, 5, 6, 8],
        vec![1, 2, 5, 6, 8, 9],
        vec![1, 2, 5, 6, 8, 9, 10],
        vec![1, 2, 5, 6, 8, 10],
        vec![1, 2, 5, 6, 9],
        vec![1, 2, 5, 6, 9, 10],
        vec![1, 2, 5, 6, 10],
        vec![1, 2, 5, 7],
        vec![1, 2, 5, 7, 8],
        vec![1, 2, 5, 7, 8, 9],
        vec![1, 2, 5, 7, 8, 9, 10],
        vec![1, 2, 5, 7, 8, 10],
        vec![1, 2, 5, 7, 9],
        vec![1, 2, 5, 7, 9, 10],
        vec![1, 2, 5, 7, 10],
        vec![1, 2, 5, 8],
        vec![1, 2, 5, 8, 9],
        vec![1, 2, 5, 8, 9, 10],
        vec![1, 2, 5, 8, 10],
        vec![1, 2, 5, 9],
        vec![1, 2, 5, 9, 10],
        vec![1, 2, 5, 10],
        vec![1, 2, 6],
        vec![1, 2, 6, 7],
        vec![1, 2, 6, 7, 8],
        vec![1, 2, 6, 7, 8, 9],
        vec![1, 2, 6, 7, 8, 9, 10],
        vec![1, 2, 6, 7, 8, 10],
        vec![1, 2, 6, 7, 9],
        vec![1, 2, 6, 7, 9, 10],
        vec![1, 2, 6, 7, 10],
        vec![1, 2, 6, 8],
        vec![1, 2, 6, 8, 9],
        vec![1, 2, 6, 8, 9, 10],
        vec![1, 2, 6, 8, 10],
        vec![1, 2, 6, 9],
        vec![1, 2, 6, 9, 10],
        vec![1, 2, 6, 10],
        vec![1, 2, 7],
        vec![1, 2, 7, 8],
        vec![1, 2, 7, 8, 9],
        vec![1, 2, 7, 8, 9, 10],
        vec![1, 2, 7, 8, 10],
        vec![1, 2, 7, 9],
        vec![1, 2, 7, 9, 10],
        vec![1, 2, 7, 10],
        vec![1, 2, 8],
        vec![1, 2, 8, 9],
        vec![1, 2, 8, 9, 10],
        vec![1, 2, 8, 10],
        vec![1, 2, 9],
        vec![1, 2, 9, 10],
        vec![1, 2, 10],
        vec![1, 3],
        vec![1, 3, 4],
        vec![1, 3, 4, 5],
        vec![1, 3, 4, 5, 6],
        vec![1, 3, 4, 5, 6, 7],
        vec![1, 3, 4, 5, 6, 7, 8],
        vec![1, 3, 4, 5, 6, 7, 8, 9],
        vec![1, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 3, 4, 5, 6, 7, 8, 10],
        vec![1, 3, 4, 5, 6, 7, 9],
        vec![1, 3, 4, 5, 6, 7, 9, 10],
        vec![1, 3, 4, 5, 6, 7, 10],
        vec![1, 3, 4, 5, 6, 8],
        vec![1, 3, 4, 5, 6, 8, 9],
        vec![1, 3, 4, 5, 6, 8, 9, 10],
        vec![1, 3, 4, 5, 6, 8, 10],
        vec![1, 3, 4, 5, 6, 9],
        vec![1, 3, 4, 5, 6, 9, 10],
        vec![1, 3, 4, 5, 6, 10],
        vec![1, 3, 4, 5, 7],
        vec![1, 3, 4, 5, 7, 8],
        vec![1, 3, 4, 5, 7, 8, 9],
        vec![1, 3, 4, 5, 7, 8, 9, 10],
        vec![1, 3, 4, 5, 7, 8, 10],
        vec![1, 3, 4, 5, 7, 9],
        vec![1, 3, 4, 5, 7, 9, 10],
        vec![1, 3, 4, 5, 7, 10],
        vec![1, 3, 4, 5, 8],
        vec![1, 3, 4, 5, 8, 9],
        vec![1, 3, 4, 5, 8, 9, 10],
        vec![1, 3, 4, 5, 8, 10],
        vec![1, 3, 4, 5, 9],
        vec![1, 3, 4, 5, 9, 10],
        vec![1, 3, 4, 5, 10],
        vec![1, 3, 4, 6],
        vec![1, 3, 4, 6, 7],
        vec![1, 3, 4, 6, 7, 8],
        vec![1, 3, 4, 6, 7, 8, 9],
        vec![1, 3, 4, 6, 7, 8, 9, 10],
        vec![1, 3, 4, 6, 7, 8, 10],
        vec![1, 3, 4, 6, 7, 9],
        vec![1, 3, 4, 6, 7, 9, 10],
        vec![1, 3, 4, 6, 7, 10],
        vec![1, 3, 4, 6, 8],
        vec![1, 3, 4, 6, 8, 9],
        vec![1, 3, 4, 6, 8, 9, 10],
        vec![1, 3, 4, 6, 8, 10],
        vec![1, 3, 4, 6, 9],
        vec![1, 3, 4, 6, 9, 10],
        vec![1, 3, 4, 6, 10],
        vec![1, 3, 4, 7],
        vec![1, 3, 4, 7, 8],
        vec![1, 3, 4, 7, 8, 9],
        vec![1, 3, 4, 7, 8, 9, 10],
        vec![1, 3, 4, 7, 8, 10],
        vec![1, 3, 4, 7, 9],
        vec![1, 3, 4, 7, 9, 10],
        vec![1, 3, 4, 7, 10],
        vec![1, 3, 4, 8],
        vec![1, 3, 4, 8, 9],
        vec![1, 3, 4, 8, 9, 10],
        vec![1, 3, 4, 8, 10],
        vec![1, 3, 4, 9],
        vec![1, 3, 4, 9, 10],
        vec![1, 3, 4, 10],
        vec![1, 3, 5],
        vec![1, 3, 5, 6],
        vec![1, 3, 5, 6, 7],
        vec![1, 3, 5, 6, 7, 8],
        vec![1, 3, 5, 6, 7, 8, 9],
        vec![1, 3, 5, 6, 7, 8, 9, 10],
        vec![1, 3, 5, 6, 7, 8, 10],
        vec![1, 3, 5, 6, 7, 9],
        vec![1, 3, 5, 6, 7, 9, 10],
        vec![1, 3, 5, 6, 7, 10],
        vec![1, 3, 5, 6, 8],
        vec![1, 3, 5, 6, 8, 9],
        vec![1, 3, 5, 6, 8, 9, 10],
        vec![1, 3, 5, 6, 8, 10],
        vec![1, 3, 5, 6, 9],
        vec![1, 3, 5, 6, 9, 10],
        vec![1, 3, 5, 6, 10],
        vec![1, 3, 5, 7],
        vec![1, 3, 5, 7, 8],
        vec![1, 3, 5, 7, 8, 9],
        vec![1, 3, 5, 7, 8, 9, 10],
        vec![1, 3, 5, 7, 8, 10],
        vec![1, 3, 5, 7, 9],
        vec![1, 3, 5, 7, 9, 10],
        vec![1, 3, 5, 7, 10],
        vec![1, 3, 5, 8],
        vec![1, 3, 5, 8, 9],
        vec![1, 3, 5, 8, 9, 10],
        vec![1, 3, 5, 8, 10],
        vec![1, 3, 5, 9],
        vec![1, 3, 5, 9, 10],
        vec![1, 3, 5, 10],
        vec![1, 3, 6],
        vec![1, 3, 6, 7],
        vec![1, 3, 6, 7, 8],
        vec![1, 3, 6, 7, 8, 9],
        vec![1, 3, 6, 7, 8, 9, 10],
        vec![1, 3, 6, 7, 8, 10],
        vec![1, 3, 6, 7, 9],
        vec![1, 3, 6, 7, 9, 10],
        vec![1, 3, 6, 7, 10],
        vec![1, 3, 6, 8],
        vec![1, 3, 6, 8, 9],
        vec![1, 3, 6, 8, 9, 10],
        vec![1, 3, 6, 8, 10],
        vec![1, 3, 6, 9],
        vec![1, 3, 6, 9, 10],
        vec![1, 3, 6, 10],
        vec![1, 3, 7],
        vec![1, 3, 7, 8],
        vec![1, 3, 7, 8, 9],
        vec![1, 3, 7, 8, 9, 10],
        vec![1, 3, 7, 8, 10],
        vec![1, 3, 7, 9],
        vec![1, 3, 7, 9, 10],
        vec![1, 3, 7, 10],
        vec![1, 3, 8],
        vec![1, 3, 8, 9],
        vec![1, 3, 8, 9, 10],
        vec![1, 3, 8, 10],
        vec![1, 3, 9],
        vec![1, 3, 9, 10],
        vec![1, 3, 10],
        vec![1, 4],
        vec![1, 4, 5],
        vec![1, 4, 5, 6],
        vec![1, 4, 5, 6, 7],
        vec![1, 4, 5, 6, 7, 8],
        vec![1, 4, 5, 6, 7, 8, 9],
        vec![1, 4, 5, 6, 7, 8, 9, 10],
        vec![1, 4, 5, 6, 7, 8, 10],
        vec![1, 4, 5, 6, 7, 9],
        vec![1, 4, 5, 6, 7, 9, 10],
        vec![1, 4, 5, 6, 7, 10],
        vec![1, 4, 5, 6, 8],
        vec![1, 4, 5, 6, 8, 9],
        vec![1, 4, 5, 6, 8, 9, 10],
        vec![1, 4, 5, 6, 8, 10],
        vec![1, 4, 5, 6, 9],
        vec![1, 4, 5, 6, 9, 10],
        vec![1, 4, 5, 6, 10],
        vec![1, 4, 5, 7],
        vec![1, 4, 5, 7, 8],
        vec![1, 4, 5, 7, 8, 9],
        vec![1, 4, 5, 7, 8, 9, 10],
        vec![1, 4, 5, 7, 8, 10],
        vec![1, 4, 5, 7, 9],
        vec![1, 4, 5, 7, 9, 10],
        vec![1, 4, 5, 7, 10],
        vec![1, 4, 5, 8],
        vec![1, 4, 5, 8, 9],
        vec![1, 4, 5, 8, 9, 10],
        vec![1, 4, 5, 8, 10],
        vec![1, 4, 5, 9],
        vec![1, 4, 5, 9, 10],
        vec![1, 4, 5, 10],
        vec![1, 4, 6],
        vec![1, 4, 6, 7],
        vec![1, 4, 6, 7, 8],
        vec![1, 4, 6, 7, 8, 9],
        vec![1, 4, 6, 7, 8, 9, 10],
        vec![1, 4, 6, 7, 8, 10],
        vec![1, 4, 6, 7, 9],
        vec![1, 4, 6, 7, 9, 10],
        vec![1, 4, 6, 7, 10],
        vec![1, 4, 6, 8],
        vec![1, 4, 6, 8, 9],
        vec![1, 4, 6, 8, 9, 10],
        vec![1, 4, 6, 8, 10],
        vec![1, 4, 6, 9],
        vec![1, 4, 6, 9, 10],
        vec![1, 4, 6, 10],
        vec![1, 4, 7],
        vec![1, 4, 7, 8],
        vec![1, 4, 7, 8, 9],
        vec![1, 4, 7, 8, 9, 10],
        vec![1, 4, 7, 8, 10],
        vec![1, 4, 7, 9],
        vec![1, 4, 7, 9, 10],
        vec![1, 4, 7, 10],
        vec![1, 4, 8],
        vec![1, 4, 8, 9],
        vec![1, 4, 8, 9, 10],
        vec![1, 4, 8, 10],
        vec![1, 4, 9],
        vec![1, 4, 9, 10],
        vec![1, 4, 10],
        vec![1, 5],
        vec![1, 5, 6],
        vec![1, 5, 6, 7],
        vec![1, 5, 6, 7, 8],
        vec![1, 5, 6, 7, 8, 9],
        vec![1, 5, 6, 7, 8, 9, 10],
        vec![1, 5, 6, 7, 8, 10],
        vec![1, 5, 6, 7, 9],
        vec![1, 5, 6, 7, 9, 10],
        vec![1, 5, 6, 7, 10],
        vec![1, 5, 6, 8],
        vec![1, 5, 6, 8, 9],
        vec![1, 5, 6, 8, 9, 10],
        vec![1, 5, 6, 8, 10],
        vec![1, 5, 6, 9],
        vec![1, 5, 6, 9, 10],
        vec![1, 5, 6, 10],
        vec![1, 5, 7],
        vec![1, 5, 7, 8],
        vec![1, 5, 7, 8, 9],
        vec![1, 5, 7, 8, 9, 10],
        vec![1, 5, 7, 8, 10],
        vec![1, 5, 7, 9],
        vec![1, 5, 7, 9, 10],
        vec![1, 5, 7, 10],
        vec![1, 5, 8],
        vec![1, 5, 8, 9],
        vec![1, 5, 8, 9, 10],
        vec![1, 5, 8, 10],
        vec![1, 5, 9],
        vec![1, 5, 9, 10],
        vec![1, 5, 10],
        vec![1, 6],
        vec![1, 6, 7],
        vec![1, 6, 7, 8],
        vec![1, 6, 7, 8, 9],
        vec![1, 6, 7, 8, 9, 10],
        vec![1, 6, 7, 8, 10],
        vec![1, 6, 7, 9],
        vec![1, 6, 7, 9, 10],
        vec![1, 6, 7, 10],
        vec![1, 6, 8],
        vec![1, 6, 8, 9],
        vec![1, 6, 8, 9, 10],
        vec![1, 6, 8, 10],
        vec![1, 6, 9],
        vec![1, 6, 9, 10],
        vec![1, 6, 10],
        vec![1, 7],
        vec![1, 7, 8],
        vec![1, 7, 8, 9],
        vec![1, 7, 8, 9, 10],
        vec![1, 7, 8, 10],
        vec![1, 7, 9],
        vec![1, 7, 9, 10],
        vec![1, 7, 10],
        vec![1, 8],
        vec![1, 8, 9],
        vec![1, 8, 9, 10],
        vec![1, 8, 10],
        vec![1, 9],
        vec![1, 9, 10],
        vec![1, 10],
        vec![1, 1],
        vec![1, 1, 1],
        vec![1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1],
        vec![2, 3],
        vec![2, 3, 4],
        vec![2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![2, 3, 4, 5, 6, 7],
        vec![2, 3, 4, 5, 6, 7, 8],
        vec![2, 3, 4, 5, 6, 7, 8, 9],
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec![2, 3, 4, 5, 6, 7, 8, 10],
        vec![2, 3, 4, 5, 6, 7, 9],
        vec![2, 3, 4, 5, 6, 7, 9, 10],
        vec![2, 3, 4, 5, 6, 7, 10],
        vec![2, 3, 4, 5, 6, 8],
        vec![2, 3, 4, 5, 6, 8, 9],
        vec![2, 3, 4, 5, 6, 8, 9, 10],
        vec![2, 3, 4, 5, 6, 8, 10],
        vec![2, 3, 4, 5, 6, 9],
        vec![2, 3, 4, 5, 6, 9, 10],
        vec![2, 3, 4, 5, 6, 10],
        vec![2, 3, 4, 5, 7],
        vec![2, 3, 4, 5, 7, 8],
        vec![2, 3, 4, 5, 7, 8, 9],
        vec![2, 3, 4, 5, 7, 8, 9, 10],
        vec![2, 3, 4, 5, 7, 8, 10],
        vec![2, 3, 4, 5, 7, 9],
        vec![2, 3, 4, 5, 7, 9, 10],
        vec![2, 3, 4, 5, 7, 10],
        vec![2, 3, 4, 5, 8],
        vec![2, 3, 4, 5, 8, 9],
        vec![2, 3, 4, 5, 8, 9, 10],
        vec![2, 3, 4, 5, 8, 10],
        vec![2, 3, 4, 5, 9],
        vec![2, 3, 4, 5, 9, 10],
        vec![2, 3, 4, 5, 10],
        vec![2, 3, 4, 6],
        vec![2, 3, 4, 6, 7],
        vec![2, 3, 4, 6, 7, 8],
        vec![2, 3, 4, 6, 7, 8, 9],
        vec![2, 3, 4, 6, 7, 8, 9, 10],
        vec![2, 3, 4, 6, 7, 8, 10],
        vec![2, 3, 4, 6, 7, 9],
        vec![2, 3, 4, 6, 7, 9, 10],
        vec![2, 3, 4, 6, 7, 10],
        vec![2, 3, 4, 6, 8],
        vec![2, 3, 4, 6, 8, 9],
        vec![2, 3, 4, 6, 8, 9, 10],
        vec![2, 3, 4, 6, 8, 10],
        vec![2, 3, 4, 6, 9],
        vec![2, 3, 4, 6, 9, 10],
        vec![2, 3, 4, 6, 10],
        vec![2, 3, 4, 7],
        vec![2, 3, 4, 7, 8],
        vec![2, 3, 4, 7, 8, 9],
        vec![2, 3, 4, 7, 8, 9, 10],
        vec![2, 3, 4, 7, 8, 10],
        vec![2, 3, 4, 7, 9],
        vec![2, 3, 4, 7, 9, 10],
        vec![2, 3, 4, 7, 10],
        vec![2, 3, 4, 8],
        vec![2, 3, 4, 8, 9],
        vec![2, 3, 4, 8, 9, 10],
        vec![2, 3, 4, 8, 10],
        vec![2, 3, 4, 9],
        vec![2, 3, 4, 9, 10],
        vec![2, 3, 4, 10],
        vec![2, 3, 5],
        vec![2, 3, 5, 6],
        vec![2, 3, 5, 6, 7],
        vec![2, 3, 5, 6, 7, 8],
        vec![2, 3, 5, 6, 7, 8, 9],
        vec![2, 3, 5, 6, 7, 8, 9, 10],
        vec![2, 3, 5, 6, 7, 8, 10],
        vec![2, 3, 5, 6, 7, 9],
        vec![2, 3, 5, 6, 7, 9, 10],
        vec![2, 3, 5, 6, 7, 10],
        vec![2, 3, 5, 6, 8],
        vec![2, 3, 5, 6, 8, 9],
        vec![2, 3, 5, 6, 8, 9, 10],
        vec![2, 3, 5, 6, 8, 10],
        vec![2, 3, 5, 6, 9],
        vec![2, 3, 5, 6, 9, 10],
        vec![2, 3, 5, 6, 10],
        vec![2, 3, 5, 7],
        vec![2, 3, 5, 7, 8],
        vec![2, 3, 5, 7, 8, 9],
        vec![2, 3, 5, 7, 8, 9, 10],
        vec![2, 3, 5, 7, 8, 10],
        vec![2, 3, 5, 7, 9],
        vec![2, 3, 5, 7, 9, 10],
        vec![2, 3, 5, 7, 10],
        vec![2, 3, 5, 8],
        vec![2, 3, 5, 8, 9],
        vec![2, 3, 5, 8, 9, 10],
        vec![2, 3, 5, 8, 10],
        vec![2, 3, 5, 9],
        vec![2, 3, 5, 9, 10],
        vec![2, 3, 5, 10],
        vec![2, 3, 6],
        vec![2, 3, 6, 7],
        vec![2, 3, 6, 7, 8],
        vec![2, 3, 6, 7, 8, 9],
        vec![2, 3, 6, 7, 8, 9, 10],
        vec![2, 3, 6, 7, 8, 10],
        vec![2, 3, 6, 7, 9],
        vec![2, 3, 6, 7, 9, 10],
        vec![2, 3, 6, 7, 10],
        vec![2, 3, 6, 8],
        vec![2, 3, 6, 8, 9],
        vec![2, 3, 6, 8, 9, 10],
        vec![2, 3, 6, 8, 10],
        vec![2, 3, 6, 9],
        vec![2, 3, 6, 9, 10],
        vec![2, 3, 6, 10],
        vec![2, 3, 7],
        vec![2, 3, 7, 8],
        vec![2, 3, 7, 8, 9],
        vec![2, 3, 7, 8, 9, 10],
        vec![2, 3, 7, 8, 10],
        vec![2, 3, 7, 9],
        vec![2, 3, 7, 9, 10],
        vec![2, 3, 7, 10],
        vec![2, 3, 8],
        vec![2, 3, 8, 9],
        vec![2, 3, 8, 9, 10],
        vec![2, 3, 8, 10],
        vec![2, 3, 9],
        vec![2, 3, 9, 10],
        vec![2, 3, 10],
        vec![2, 4],
        vec![2, 4, 5],
        vec![2, 4, 5, 6],
        vec![2, 4, 5, 6, 7],
        vec![2, 4, 5, 6, 7, 8],
        vec![2, 4, 5, 6, 7, 8, 9],
        vec![2, 4, 5, 6, 7, 8, 9, 10],
        vec![2, 4, 5, 6, 7, 8, 10],
        vec![2, 4, 5, 6, 7, 9],
        vec![2, 4, 5, 6, 7, 9, 10],
        vec![2, 4, 5, 6, 7, 10],
        vec![2, 4, 5, 6, 8],
        vec![2, 4, 5, 6, 8, 9],
        vec![2, 4, 5, 6, 8, 9, 10],
        vec![2, 4, 5, 6, 8, 10],
        vec![2, 4, 5, 6, 9],
        vec![2, 4, 5, 6, 9, 10],
        vec![2, 4, 5, 6, 10],
        vec![2, 4, 5, 7],
        vec![2, 4, 5, 7, 8],
        vec![2, 4, 5, 7, 8, 9],
        vec![2, 4, 5, 7, 8, 9, 10],
        vec![2, 4, 5, 7, 8, 10],
        vec![2, 4, 5, 7, 9],
        vec![2, 4, 5, 7, 9, 10],
        vec![2, 4, 5, 7, 10],
        vec![2, 4, 5, 8],
        vec![2, 4, 5, 8, 9],
        vec![2, 4, 5, 8, 9, 10],
        vec![2, 4, 5, 8, 10],
        vec![2, 4, 5, 9],
        vec![2, 4, 5, 9, 10],
        vec![2, 4, 5, 10],
        vec![2, 4, 6],
        vec![2, 4, 6, 7],
        vec![2, 4, 6, 7, 8],
        vec![2, 4, 6, 7, 8, 9],
        vec![2, 4, 6, 7, 8, 9, 10],
        vec![2, 4, 6, 7, 8, 10],
        vec![2, 4, 6, 7, 9],
        vec![2, 4, 6, 7, 9, 10],
        vec![2, 4, 6, 7, 10],
        vec![2, 4, 6, 8],
        vec![2, 4, 6, 8, 9],
        vec![2, 4, 6, 8, 9, 10],
        vec![2, 4, 6, 8, 10],
        vec![2, 4, 6, 9],
        vec![2, 4, 6, 9, 10],
        vec![2, 4, 6, 10],
        vec![2, 4, 7],
        vec![2, 4, 7, 8],
        vec![2, 4, 7, 8, 9],
        vec![2, 4, 7, 8, 9, 10],
        vec![2, 4, 7, 8, 10],
        vec![2, 4, 7, 9],
        vec![2, 4, 7, 9, 10],
        vec![2, 4, 7, 10],
        vec![2, 4, 8],
        vec![2, 4, 8, 9],
        vec![2, 4, 8, 9, 10],
        vec![2, 4, 8, 10],
        vec![2, 4, 9],
        vec![2, 4, 9, 10],
        vec![2, 4, 10],
        vec![2, 5],
        vec![2, 5, 6],
        vec![2, 5, 6, 7],
        vec![2, 5, 6, 7, 8],
        vec![2, 5, 6, 7, 8, 9],
        vec![2, 5, 6, 7, 8, 9, 10],
        vec![2, 5, 6, 7, 8, 10],
        vec![2, 5, 6, 7, 9],
        vec![2, 5, 6, 7, 9, 10],
        vec![2, 5, 6, 7, 10],
        vec![2, 5, 6, 8],
        vec![2, 5, 6, 8, 9],
        vec![2, 5, 6, 8, 9, 10],
        vec![2, 5, 6, 8, 10],
        vec![2, 5, 6, 9],
        vec![2, 5, 6, 9, 10],
        vec![2, 5, 6, 10],
        vec![2, 5, 7],
        vec![2, 5, 7, 8],
        vec![2, 5, 7, 8, 9],
        vec![2, 5, 7, 8, 9, 10],
        vec![2, 5, 7, 8, 10],
        vec![2, 5, 7, 9],
        vec![2, 5, 7, 9, 10],
        vec![2, 5, 7, 10],
        vec![2, 5, 8],
        vec![2, 5, 8, 9],
        vec![2, 5, 8, 9, 10],
        vec![2, 5, 8, 10],
        vec![2, 5, 9],
        vec![2, 5, 9, 10],
        vec![2, 5, 10],
        vec![2, 6],
        vec![2, 6, 7],
        vec![2, 6, 7, 8],
        vec![2, 6, 7, 8, 9],
        vec![2, 6, 7, 8, 9, 10],
        vec![2, 6, 7, 8, 10],
        vec![2, 6, 7, 9],
        vec![2, 6, 7, 9, 10],
        vec![2, 6, 7, 10],
        vec![2, 6, 8],
        vec![2, 6, 8, 9],
        vec![2, 6, 8, 9, 10],
        vec![2, 6, 8, 10],
        vec![2, 6, 9],
        vec![2, 6, 9, 10],
        vec![2, 6, 10],
        vec![2, 7],
        vec![2, 7, 8],
        vec![2, 7, 8, 9],
        vec![2, 7, 8, 9, 10],
        vec![2, 7, 8, 10],
        vec![2, 7, 9],
        vec![2, 7, 9, 10],
        vec![2, 7, 10],
        vec![2, 8],
        vec![2, 8, 9],
        vec![2, 8, 9, 10],
        vec![2, 8, 10],
        vec![2, 9],
        vec![2, 9, 10],
        vec![2, 10],
        vec![3, 4],
        vec![3, 4, 5],
        vec![3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![3, 4, 5, 6, 7, 8],
        vec![3, 4, 5, 6, 7, 8, 9],
        vec![3, 4, 5, 6, 7, 8, 9, 10],
        vec![3, 4, 5, 6, 7, 8, 10],
        vec![3, 4, 5, 6, 7, 9],
        vec![3, 4, 5, 6, 7, 9, 10],
        vec![3, 4, 5, 6, 7, 10],
        vec![3, 4, 5, 6, 8],
        vec![3, 4, 5, 6, 8, 9],
        vec![3, 4, 5, 6, 8, 9, 10],
        vec![3, 4, 5, 6, 8, 10],
        vec![3, 4, 5, 6, 9],
        vec![3, 4, 5, 6, 9, 10],
        vec![3, 4, 5, 6, 10],
        vec![3, 4, 5, 7],
        vec![3, 4, 5, 7, 8],
        vec![3, 4, 5, 7, 8, 9],
        vec![3, 4, 5, 7, 8, 9, 10],
        vec![3, 4, 5, 7, 8, 10],
        vec![3, 4, 5, 7, 9],
        vec![3, 4, 5, 7, 9, 10],
        vec![3, 4, 5, 7, 10],
        vec![3, 4, 5, 8],
        vec![3, 4, 5, 8, 9],
        vec![3, 4, 5, 8, 9, 10],
        vec![3, 4, 5, 8, 10],
        vec![3, 4, 5, 9],
        vec![3, 4, 5, 9, 10],
        vec![3, 4, 5, 10],
        vec![3, 4, 6],
        vec![3, 4, 6, 7],
        vec![3, 4, 6, 7, 8],
        vec![3, 4, 6, 7, 8, 9],
        vec![3, 4, 6, 7, 8, 9, 10],
        vec![3, 4, 6, 7, 8, 10],
        vec![3, 4, 6, 7, 9],
        vec![3, 4, 6, 7, 9, 10],
        vec![3, 4, 6, 7, 10],
        vec![3, 4, 6, 8],
        vec![3, 4, 6, 8, 9],
        vec![3, 4, 6, 8, 9, 10],
        vec![3, 4, 6, 8, 10],
        vec![3, 4, 6, 9],
        vec![3, 4, 6, 9, 10],
        vec![3, 4, 6, 10],
        vec![3, 4, 7],
        vec![3, 4, 7, 8],
        vec![3, 4, 7, 8, 9],
        vec![3, 4, 7, 8, 9, 10],
        vec![3, 4, 7, 8, 10],
        vec![3, 4, 7, 9],
        vec![3, 4, 7, 9, 10],
        vec![3, 4, 7, 10],
        vec![3, 4, 8],
        vec![3, 4, 8, 9],
        vec![3, 4, 8, 9, 10],
        vec![3, 4, 8, 10],
        vec![3, 4, 9],
        vec![3, 4, 9, 10],
        vec![3, 4, 10],
        vec![3, 5],
        vec![3, 5, 6],
        vec![3, 5, 6, 7],
        vec![3, 5, 6, 7, 8],
        vec![3, 5, 6, 7, 8, 9],
        vec![3, 5, 6, 7, 8, 9, 10],
        vec![3, 5, 6, 7, 8, 10],
        vec![3, 5, 6, 7, 9],
        vec![3, 5, 6, 7, 9, 10],
        vec![3, 5, 6, 7, 10],
        vec![3, 5, 6, 8],
        vec![3, 5, 6, 8, 9],
        vec![3, 5, 6, 8, 9, 10],
        vec![3, 5, 6, 8, 10],
        vec![3, 5, 6, 9],
        vec![3, 5, 6, 9, 10],
        vec![3, 5, 6, 10],
        vec![3, 5, 7],
        vec![3, 5, 7, 8],
        vec![3, 5, 7, 8, 9],
        vec![3, 5, 7, 8, 9, 10],
        vec![3, 5, 7, 8, 10],
        vec![3, 5, 7, 9],
        vec![3, 5, 7, 9, 10],
        vec![3, 5, 7, 10],
        vec![3, 5, 8],
        vec![3, 5, 8, 9],
        vec![3, 5, 8, 9, 10],
        vec![3, 5, 8, 10],
        vec![3, 5, 9],
        vec![3, 5, 9, 10],
        vec![3, 5, 10],
        vec![3, 6],
        vec![3, 6, 7],
        vec![3, 6, 7, 8],
        vec![3, 6, 7, 8, 9],
        vec![3, 6, 7, 8, 9, 10],
        vec![3, 6, 7, 8, 10],
        vec![3, 6, 7, 9],
        vec![3, 6, 7, 9, 10],
        vec![3, 6, 7, 10],
        vec![3, 6, 8],
        vec![3, 6, 8, 9],
        vec![3, 6, 8, 9, 10],
        vec![3, 6, 8, 10],
        vec![3, 6, 9],
        vec![3, 6, 9, 10],
        vec![3, 6, 10],
        vec![3, 7],
        vec![3, 7, 8],
        vec![3, 7, 8, 9],
        vec![3, 7, 8, 9, 10],
        vec![3, 7, 8, 10],
        vec![3, 7, 9],
        vec![3, 7, 9, 10],
        vec![3, 7, 10],
        vec![3, 8],
        vec![3, 8, 9],
        vec![3, 8, 9, 10],
        vec![3, 8, 10],
        vec![3, 9],
        vec![3, 9, 10],
        vec![3, 10],
        vec![4, 5],
        vec![4, 5, 6],
        vec![4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
        vec![4, 5, 6, 7, 8, 9],
        vec![4, 5, 6, 7, 8, 9, 10],
        vec![4, 5, 6, 7, 8, 10],
        vec![4, 5, 6, 7, 9],
        vec![4, 5, 6, 7, 9, 10],
        vec![4, 5, 6, 7, 10],
        vec![4, 5, 6, 8],
        vec![4, 5, 6, 8, 9],
        vec![4, 5, 6, 8, 9, 10],
        vec![4, 5, 6, 8, 10],
        vec![4, 5, 6, 9],
        vec![4, 5, 6, 9, 10],
        vec![4, 5, 6, 10],
        vec![4, 5, 7],
        vec![4, 5, 7, 8],
        vec![4, 5, 7, 8, 9],
        vec![4, 5, 7, 8, 9, 10],
        vec![4, 5, 7, 8, 10],
        vec![4, 5, 7, 9],
        vec![4, 5, 7, 9, 10],
        vec![4, 5, 7, 10],
        vec![4, 5, 8],
        vec![4, 5, 8, 9],
        vec![4, 5, 8, 9, 10],
        vec![4, 5, 8, 10],
        vec![4, 5, 9],
        vec![4, 5, 9, 10],
        vec![4, 5, 10],
        vec![4, 6],
        vec![4, 6, 7],
        vec![4, 6, 7, 8],
        vec![4, 6, 7, 8, 9],
        vec![4, 6, 7, 8, 9, 10],
        vec![4, 6, 7, 8, 10],
        vec![4, 6, 7, 9],
        vec![4, 6, 7, 9, 10],
        vec![4, 6, 7, 10],
        vec![4, 6, 8],
        vec![4, 6, 8, 9],
        vec![4, 6, 8, 9, 10],
        vec![4, 6, 8, 10],
        vec![4, 6, 9],
        vec![4, 6, 9, 10],
        vec![4, 6, 10],
        vec![4, 7],
        vec![4, 7, 8],
        vec![4, 7, 8, 9],
        vec![4, 7, 8, 9, 10],
        vec![4, 7, 8, 10],
        vec![4, 7, 9],
        vec![4, 7, 9, 10],
        vec![4, 7, 10],
        vec![4, 8],
        vec![4, 8, 9],
        vec![4, 8, 9, 10],
        vec![4, 8, 10],
        vec![4, 9],
        vec![4, 9, 10],
        vec![4, 10],
        vec![5, 6],
        vec![5, 6, 7],
        vec![5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
        vec![5, 6, 7, 8, 9, 10],
        vec![5, 6, 7, 8, 10],
        vec![5, 6, 7, 9],
        vec![5, 6, 7, 9, 10],
        vec![5, 6, 7, 10],
        vec![5, 6, 8],
        vec![5, 6, 8, 9],
        vec![5, 6, 8, 9, 10],
        vec![5, 6, 8, 10],
        vec![5, 6, 9],
        vec![5, 6, 9, 10],
        vec![5, 6, 10],
        vec![5, 7],
        vec![5, 7, 8],
        vec![5, 7, 8, 9],
        vec![5, 7, 8, 9, 10],
        vec![5, 7, 8, 10],
        vec![5, 7, 9],
        vec![5, 7, 9, 10],
        vec![5, 7, 10],
        vec![5, 8],
        vec![5, 8, 9],
        vec![5, 8, 9, 10],
        vec![5, 8, 10],
        vec![5, 9],
        vec![5, 9, 10],
        vec![5, 10],
        vec![6, 7],
        vec![6, 7, 8],
        vec![6, 7, 8, 9],
        vec![6, 7, 8, 9, 10],
        vec![6, 7, 8, 10],
        vec![6, 7, 9],
        vec![6, 7, 9, 10],
        vec![6, 7, 10],
        vec![6, 8],
        vec![6, 8, 9],
        vec![6, 8, 9, 10],
        vec![6, 8, 10],
        vec![6, 9],
        vec![6, 9, 10],
        vec![6, 10],
        vec![7, 8],
        vec![7, 8, 9],
        vec![7, 8, 9, 10],
        vec![7, 8, 10],
        vec![7, 9],
        vec![7, 9, 10],
        vec![7, 10],
        vec![8, 9],
        vec![8, 9, 10],
        vec![8, 10],
        vec![9, 10],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn non_decreasing_subsequences_4() {
    let nums = vec![100, 90, 80, 70, 60, 50, 60, 70, 80, 90, 100];
    let res = Solution::find_subsequences(nums);

    assert_eq!(res.len(), 88);
    let set = HashSet::from([
        vec![100, 100],
        vec![90, 90],
        vec![90, 90, 100],
        vec![90, 100],
        vec![80, 80],
        vec![80, 80, 90],
        vec![80, 80, 90, 100],
        vec![80, 80, 100],
        vec![80, 90],
        vec![80, 90, 100],
        vec![80, 100],
        vec![70, 70],
        vec![70, 70, 80],
        vec![70, 70, 80, 90],
        vec![70, 70, 80, 90, 100],
        vec![70, 70, 80, 100],
        vec![70, 70, 90],
        vec![70, 70, 90, 100],
        vec![70, 70, 100],
        vec![70, 80],
        vec![70, 80, 90],
        vec![70, 80, 90, 100],
        vec![70, 80, 100],
        vec![70, 90],
        vec![70, 90, 100],
        vec![70, 100],
        vec![60, 60],
        vec![60, 60, 70],
        vec![60, 60, 70, 80],
        vec![60, 60, 70, 80, 90],
        vec![60, 60, 70, 80, 90, 100],
        vec![60, 60, 70, 80, 100],
        vec![60, 60, 70, 90],
        vec![60, 60, 70, 90, 100],
        vec![60, 60, 70, 100],
        vec![60, 60, 80],
        vec![60, 60, 80, 90],
        vec![60, 60, 80, 90, 100],
        vec![60, 60, 80, 100],
        vec![60, 60, 90],
        vec![60, 60, 90, 100],
        vec![60, 60, 100],
        vec![60, 70],
        vec![60, 70, 80],
        vec![60, 70, 80, 90],
        vec![60, 70, 80, 90, 100],
        vec![60, 70, 80, 100],
        vec![60, 70, 90],
        vec![60, 70, 90, 100],
        vec![60, 70, 100],
        vec![60, 80],
        vec![60, 80, 90],
        vec![60, 80, 90, 100],
        vec![60, 80, 100],
        vec![60, 90],
        vec![60, 90, 100],
        vec![60, 100],
        vec![50, 60],
        vec![50, 60, 70],
        vec![50, 60, 70, 80],
        vec![50, 60, 70, 80, 90],
        vec![50, 60, 70, 80, 90, 100],
        vec![50, 60, 70, 80, 100],
        vec![50, 60, 70, 90],
        vec![50, 60, 70, 90, 100],
        vec![50, 60, 70, 100],
        vec![50, 60, 80],
        vec![50, 60, 80, 90],
        vec![50, 60, 80, 90, 100],
        vec![50, 60, 80, 100],
        vec![50, 60, 90],
        vec![50, 60, 90, 100],
        vec![50, 60, 100],
        vec![50, 70],
        vec![50, 70, 80],
        vec![50, 70, 80, 90],
        vec![50, 70, 80, 90, 100],
        vec![50, 70, 80, 100],
        vec![50, 70, 90],
        vec![50, 70, 90, 100],
        vec![50, 70, 100],
        vec![50, 80],
        vec![50, 80, 90],
        vec![50, 80, 90, 100],
        vec![50, 80, 100],
        vec![50, 90],
        vec![50, 90, 100],
        vec![50, 100],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
