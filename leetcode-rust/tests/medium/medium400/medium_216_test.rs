use leetcode_exercise::leetcode::editor::cn::_216_combination_sum_i_i_i::Solution;
use std::collections::HashSet;

#[test]
fn combination_sum_iii_1() {
    let (k, n) = (3, 7);
    let res = Solution::combination_sum3(k, n);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec![1, 2, 4]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn combination_sum_iii_2() {
    let (k, n) = (3, 9);
    let res = Solution::combination_sum3(k, n);

    assert_eq!(res.len(), 3);
    let set = HashSet::from([vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn combination_sum_iii_3() {
    let (k, n) = (4, 1);
    let res = Solution::combination_sum3(k, n);

    assert_eq!(res.is_empty(), true);
}
