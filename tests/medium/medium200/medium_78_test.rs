use leetcode_exercise::leetcode::editor::cn::_78_subsets::Solution;
use std::collections::HashSet;

#[test]
fn subsets_1() {
    let nums = vec![1, 2, 3];
    let len = nums.len();
    let res = Solution::subsets(nums);

    assert_eq!(res.len(), 2_usize.pow(len as u32));
    let set = HashSet::from([
        vec![],
        vec![1],
        vec![2],
        vec![1, 2],
        vec![3],
        vec![1, 3],
        vec![2, 3],
        vec![1, 2, 3],
    ]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}

#[test]
fn subsets_2() {
    let nums = vec![0];
    let len = nums.len();
    let res = Solution::subsets(nums);

    assert_eq!(res.len(), 2_usize.pow(len as u32));
    let set = HashSet::from([vec![], vec![0]]);
    for item in res {
        assert_eq!(set.contains(&item), true);
    }
}
