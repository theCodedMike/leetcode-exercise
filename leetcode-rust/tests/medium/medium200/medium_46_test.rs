use leetcode_rust::leetcode::editor::cn::_46_permutations::Solution;
use std::collections::HashSet;

#[test]
fn permutations_1() {
    let nums = vec![1, 2, 3];
    let res = Solution::permute(nums);

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
fn permutations_2() {
    let nums = vec![0, 1];
    let res = Solution::permute(nums);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([vec![0, 1], vec![1, 0]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn permutations_3() {
    let nums = vec![1];
    let res = Solution::permute(nums);

    assert_eq!(res.len(), 1);
    let set = HashSet::from([vec![1]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
