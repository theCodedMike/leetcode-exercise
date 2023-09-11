use leetcode_exercise::leetcode::editor::en::_162_find_peak_element::Solution;

#[test]
fn find_peak_element() {
    let nums = vec![1, 3, 2, 1];
    let idx = Solution::find_peak_element(nums);
    assert_eq!(idx, 1);
}
