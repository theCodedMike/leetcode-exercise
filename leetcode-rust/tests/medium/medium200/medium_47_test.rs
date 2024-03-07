use leetcode_rust::leetcode::editor::cn::_47_permutations_i_i::Solution;
use std::collections::HashSet;

#[test]
fn permutations_ii_1() {
    let nums = vec![1, 1, 2];
    let res = Solution::permute_unique(nums);

    assert_eq!(res.len(), 3);
    let set = HashSet::from([vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn permutations_ii_2() {
    let nums = vec![1, 2, 3];
    let res = Solution::permute_unique(nums);

    assert_eq!(res.len(), 6);
    let set = HashSet::from([
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn permutations_ii_3() {
    let nums = vec![3, 3, 0, 3];
    let res = Solution::permute_unique(nums);

    assert_eq!(res.len(), 4);
    let set = HashSet::from([
        vec![0, 3, 3, 3],
        vec![3, 0, 3, 3],
        vec![3, 3, 0, 3],
        vec![3, 3, 3, 0],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
