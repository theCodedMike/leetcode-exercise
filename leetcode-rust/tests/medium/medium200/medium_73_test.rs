use leetcode_rust::leetcode::editor::cn::_73_set_matrix_zeroes::Solution;

#[test]
fn set_matrix_zeroes() {
    let mut matrix = vec![
        vec![0, 0, 0, 5],
        vec![4, 3, 1, 4],
        vec![0, 1, 1, 4],
        vec![1, 2, 1, 3],
        vec![0, 0, 1, 1],
    ];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(
        matrix,
        [
            [0, 0, 0, 0],
            [0, 0, 0, 4],
            [0, 0, 0, 0],
            [0, 0, 0, 3],
            [0, 0, 0, 0]
        ]
    );
}
