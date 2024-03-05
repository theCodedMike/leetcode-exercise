use leetcode_exercise::leetcode::editor::cn::_35_search_insert_position::Solution;

#[test]
fn search_insert_position() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    let pos = Solution::search_insert(nums, target);
    assert_eq!(pos, 2);

    let nums = vec![1, 3, 5, 6];
    let target = 0;
    let pos = Solution::search_insert(nums, target);
    assert_eq!(pos, 0);
}
