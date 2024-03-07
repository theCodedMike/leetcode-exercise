use leetcode_rust::leetcode::editor::cn::_119_pascals_triangle_i_i::Solution;

#[test]
fn pascal_triangle_ii() {
    let row_index = 4;
    let res = Solution::get_row(row_index);
    assert_eq!(res, [1, 4, 6, 4, 1]);

    let row_index = 30;
    let res = Solution::get_row(row_index);
    assert_eq!(
        res,
        [
            1, 30, 435, 4060, 27405, 142506, 593775, 2035800, 5852925, 14307150, 30045015,
            54627300, 86493225, 119759850, 145422675, 155117520, 145422675, 119759850, 86493225,
            54627300, 30045015, 14307150, 5852925, 2035800, 593775, 142506, 27405, 4060, 435, 30,
            1
        ]
    )
}
