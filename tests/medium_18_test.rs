use leetcode_exercise::leetcode::editor::en::_18_4_sum::Solution;

#[test]
fn _4_sum_test() {
    let nums = vec![1, 0, -1, 0, -2, 2];
    let target = 0;
    let res = Solution::four_sum(nums, target);
    assert_eq!(res, [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]);

    let nums = vec![2, 2, 2, 2, 2];
    let target = 8;
    let res = Solution::four_sum(nums, target);
    assert_eq!(res, [[2, 2, 2, 2]]);

    let nums = vec![-3, -1, 0, 2, 4, 5];
    let target = 0;
    let res = Solution::four_sum(nums, target);
    assert_eq!(res, [[-3, -1, 0, 4]]);

    let nums = vec![1000000000, 1000000000, 1000000000, 1000000000];
    let target = -294967296;
    let res = Solution::four_sum(nums, target);
    assert_eq!(res, [[]; 0]);

    let nums = vec![-3, -2, -1, 0, 0, 1, 2, 3];
    let target = 0;
    let res = Solution::four_sum(nums, target);
    assert_eq!(
        res,
        [
            [-3, -2, 2, 3],
            [-3, -1, 1, 3],
            [-3, 0, 0, 3],
            [-3, 0, 1, 2],
            [-2, -1, 0, 3],
            [-2, -1, 1, 2],
            [-2, 0, 0, 2],
            [-1, 0, 0, 1]
        ]
    );

    let nums = vec![-5, -4, -3, -2, -1, 0, 0, 1, 2, 3, 4, 5];
    let target = 0;
    let res = Solution::four_sum(nums, target);
    assert_eq!(
        res,
        [
            [-5, -4, 4, 5],
            [-5, -3, 3, 5],
            [-5, -2, 2, 5],
            [-5, -2, 3, 4],
            [-5, -1, 1, 5],
            [-5, -1, 2, 4],
            [-5, 0, 0, 5],
            [-5, 0, 1, 4],
            [-5, 0, 2, 3],
            [-4, -3, 2, 5],
            [-4, -3, 3, 4],
            [-4, -2, 1, 5],
            [-4, -2, 2, 4],
            [-4, -1, 0, 5],
            [-4, -1, 1, 4],
            [-4, -1, 2, 3],
            [-4, 0, 0, 4],
            [-4, 0, 1, 3],
            [-3, -2, 0, 5],
            [-3, -2, 1, 4],
            [-3, -2, 2, 3],
            [-3, -1, 0, 4],
            [-3, -1, 1, 3],
            [-3, 0, 0, 3],
            [-3, 0, 1, 2],
            [-2, -1, 0, 3],
            [-2, -1, 1, 2],
            [-2, 0, 0, 2],
            [-1, 0, 0, 1]
        ]
    );
}
