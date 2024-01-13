use leetcode_exercise::leetcode::editor::cn::_704_binary_search::Solution;

#[test]
fn binary_search() {
    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 9;
    let res = Solution::search(nums, target);
    assert_eq!(res, 4);

    let nums = vec![-1, 0, 3, 5, 9, 12];
    let target = 2;
    let res = Solution::search(nums, target);
    assert_eq!(res, -1);

    let nums = vec![5];
    let target = -5;
    let res = Solution::search(nums, target);
    assert_eq!(res, -1);
}
