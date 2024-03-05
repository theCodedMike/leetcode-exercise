use leetcode_exercise::leetcode::editor::cn::_88_merge_sorted_array::Solution;

#[test]
fn merge_sorted_array() {
    let mut vec1 = vec![2, 0];
    let len1 = 1;

    let mut vec2 = vec![1];
    let len2 = 1;

    Solution::merge(&mut vec1, len1, &mut vec2, len2);

    assert_eq!(vec1, vec![1, 2]);
}
