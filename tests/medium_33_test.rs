use leetcode_exercise::leetcode::editor::en::_33_search_in_rotated_sorted_array::Solution;

#[test]
fn search_in_rotated_sorted_array() {
    let array = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    let res = Solution::search(array, target);
    assert_eq!(res, 4);

    let array = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    let res = Solution::search(array, target);
    assert_eq!(res, -1);

    let array = vec![1];
    let target = 0;
    let res = Solution::search(array, target);
    assert_eq!(res, -1);
}
