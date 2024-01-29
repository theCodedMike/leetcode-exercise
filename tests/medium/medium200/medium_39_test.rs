use leetcode_exercise::leetcode::editor::cn::_39_combination_sum::Solution;
use std::collections::HashSet;

#[test]
fn combination_sum_1() {
    let candidates = vec![2, 3, 6, 7];
    let target = 7;
    let res = Solution::combination_sum(candidates, target);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([vec![2, 2, 3], vec![7]]);
    for mut item in res {
        item.sort_unstable();
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn combination_sum_2() {
    let candidates = vec![2, 3, 5];
    let target = 8;
    let res = Solution::combination_sum(candidates, target);

    assert_eq!(res.len(), 3);
    let set = HashSet::from([vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
    for mut item in res {
        item.sort_unstable();
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn combination_sum_3() {
    let candidates = vec![2];
    let target = 1;
    let res = Solution::combination_sum(candidates, target);

    assert_eq!(res.is_empty(), true);
}

#[test]
fn combination_sum_4() {
    let candidates = vec![8, 7, 4, 3];
    let target = 11;
    let res = Solution::combination_sum(candidates, target);

    assert_eq!(res.len(), 3);
    let set = HashSet::from([vec![3, 8], vec![4, 7], vec![3, 4, 4]]);
    for mut item in res {
        item.sort_unstable();
        assert_eq!(set.contains(&item), true);
    }
}
