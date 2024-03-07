use leetcode_rust::leetcode::editor::cn::_77_combinations::Solution;
use std::collections::HashSet;

#[test]
fn combinations_1() {
    let (n, k) = (4, 2);
    let res = Solution::combine(n, k);

    assert_eq!(res.len(), 6);
    let set = HashSet::from([
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ]);

    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn combinations_2() {
    let (n, k) = (1, 1);
    let res = Solution::combine(n, k);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec![1]]);

    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
