use leetcode_rust::leetcode::editor::cn::_90_subsets_i_i::Solution;
use std::collections::HashSet;

#[test]
fn subsets_ii_1() {
    let nums = vec![1, 2, 2];
    let res = Solution::subsets_with_dup(nums);

    assert_eq!(res.len(), 6);
    let set = HashSet::from([
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 2],
        vec![2],
        vec![2, 2],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn subsets_ii_2() {
    let nums = vec![0];
    let res = Solution::subsets_with_dup(nums);

    assert_eq!(res.len(), 2);
    let set = HashSet::from([vec![], vec![0]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn subsets_ii_3() {
    let nums = vec![4, 4, 4, 1, 4];
    let res = Solution::subsets_with_dup(nums);

    assert_eq!(res.len(), 10);
    let set = HashSet::from([
        vec![],
        vec![1],
        vec![1, 4],
        vec![1, 4, 4],
        vec![1, 4, 4, 4],
        vec![1, 4, 4, 4, 4],
        vec![4],
        vec![4, 4],
        vec![4, 4, 4],
        vec![4, 4, 4, 4],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn subsets_ii_4() {
    let nums = vec![1, 2, 3, 5, 5, 5];
    let res = Solution::subsets_with_dup(nums);

    assert_eq!(res.len(), 32);
    let set = HashSet::from([
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
        vec![5],
        vec![1, 5],
        vec![2, 5],
        vec![1, 2, 5],
        vec![3, 5],
        vec![1, 3, 5],
        vec![2, 3, 5],
        vec![1, 2, 3, 5],
        vec![5, 5],
        vec![1, 5, 5],
        vec![2, 5, 5],
        vec![1, 2, 5, 5],
        vec![3, 5, 5],
        vec![1, 3, 5, 5],
        vec![2, 3, 5, 5],
        vec![1, 2, 3, 5, 5],
        vec![5, 5, 5],
        vec![1, 5, 5, 5],
        vec![2, 5, 5, 5],
        vec![1, 2, 5, 5, 5],
        vec![3, 5, 5, 5],
        vec![1, 3, 5, 5, 5],
        vec![2, 3, 5, 5, 5],
        vec![1, 2, 3, 5, 5, 5],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
