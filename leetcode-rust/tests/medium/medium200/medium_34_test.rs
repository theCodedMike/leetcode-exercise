use leetcode_rust::leetcode::editor::cn::_34_find_first_and_last_position_of_element_in_sorted_array::Solution;

#[test]
fn find_first_and_last_position_of_element_in_sorted_array() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [3, 4]);

    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [-1, -1]);

    let nums = vec![];
    let target = 0;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [-1, -1]);

    let nums = vec![1];
    let target = 0;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [-1, -1]);

    let nums = vec![1];
    let target = 1;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [0, 0]);

    let nums = vec![2, 2];
    let target = 2;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [0, 1]);

    let nums = vec![1, 3];
    let target = 1;
    let res = Solution::search_range(nums, target);
    assert_eq!(res, [0, 0]);
}
