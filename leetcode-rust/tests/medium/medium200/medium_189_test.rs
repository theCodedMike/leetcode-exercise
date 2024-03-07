use leetcode_rust::leetcode::editor::cn::_189_rotate_array::Solution;

#[test]
fn rotate_array() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    assert_eq!(nums, [5, 6, 7, 1, 2, 3, 4]);

    let mut nums = vec![-1, -100, 3, 99];
    let k = 2;
    Solution::rotate(&mut nums, k);
    assert_eq!(nums, [3, 99, -1, -100]);

    let mut nums = vec![1, 2, 3];
    let k = 2;
    Solution::rotate(&mut nums, k);
    assert_eq!(nums, [2, 3, 1]);
}
