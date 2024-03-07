use leetcode_rust::leetcode::editor::cn::_40_combination_sum_i_i::Solution;
use std::collections::HashSet;

#[test]
fn combination_sum_ii_1() {
    let candidates = vec![10, 1, 2, 7, 6, 1, 5];
    let target = 8;
    let res = Solution::combination_sum2(candidates, target);

    assert_eq!(res.len(), 4);
    let set = HashSet::from([vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]);
    for item in res {
        assert_eq!(set.contains(&item), true)
    }
}

#[test]
fn combination_sum_ii_2() {
    let candidates = vec![2, 5, 2, 1, 2];
    let target = 5;
    let res = Solution::combination_sum2(candidates, target);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([vec![1, 2, 2], vec![5]]);
    for item in res {
        assert_eq!(set.contains(&item), true)
    }
}

#[test]
fn combination_sum_ii_3() {
    let candidates = vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];
    let target = 30;
    let res = Solution::combination_sum2(candidates, target);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec![
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ]]);
    for item in res {
        assert_eq!(set.contains(&item), true)
    }
}
