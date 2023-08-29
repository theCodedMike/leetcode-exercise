use leetcode_exercise::leetcode::editor::en::_130_surrounded_regions::Solution;

#[test]
fn surrounded_regions() {
    let mut board = vec![
        vec!['O', 'X', 'X', 'O', 'X'],
        vec!['X', 'O', 'O', 'X', 'O'],
        vec!['X', 'O', 'X', 'O', 'X'],
        vec!['O', 'X', 'O', 'O', 'O'],
        vec!['X', 'X', 'O', 'X', 'O'],
    ];
    Solution::solve(&mut board);
    let expected_res = [
        ['O', 'X', 'X', 'O', 'X'],
        ['X', 'X', 'X', 'X', 'O'],
        ['X', 'X', 'X', 'O', 'X'],
        ['O', 'X', 'O', 'O', 'O'],
        ['X', 'X', 'O', 'X', 'O'],
    ];
    assert_eq!(board, expected_res);
}
