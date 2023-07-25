use leetcode_exercise::leetcode::editor::en::_59_spiral_matrix_i_i::Solution;

#[test]
fn spiral_matrix_ii() {
    let vec = Solution::generate_matrix(3);
    assert_eq!(vec, [[1, 2, 3], [8, 9, 4], [7, 6, 5]]);
}
