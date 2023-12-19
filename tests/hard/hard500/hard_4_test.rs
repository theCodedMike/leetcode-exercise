use leetcode_exercise::leetcode::editor::en::_4_median_of_two_sorted_arrays::Solution;

#[test]
fn median_of_two_sorted_arrays() {
    let vec1 = vec![1, 3];
    let vec2 = vec![2];

    let res = Solution::find_median_sorted_arrays(vec1, vec2);
    assert_eq!(2.0, res);
}
