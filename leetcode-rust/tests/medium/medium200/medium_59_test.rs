use leetcode_exercise::leetcode::editor::cn::_59_spiral_matrix_i_i::Solution;

#[test]
fn spiral_matrix_ii() {
    let vec = Solution::generate_matrix(1);
    assert_eq!(vec, [[1]]);

    let vec = Solution::generate_matrix(2);
    assert_eq!(vec, [[1, 2], [4, 3]]);

    let vec = Solution::generate_matrix(3);
    assert_eq!(vec, [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);

    let vec = Solution::generate_matrix(4);
    assert_eq!(
        vec,
        [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7]
        ]
    );
}
