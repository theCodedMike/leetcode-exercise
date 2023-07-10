//Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be
//validated according to the following rules:
//
//
// Each row must contain the digits 1-9 without repetition.
// Each column must contain the digits 1-9 without repetition.
// Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9
//without repetition.
//
//
// Note:
//
//
// A Sudoku board (partially filled) could be valid but is not necessarily
//solvable.
// Only the filled cells need to be validated according to the mentioned rules.
//
//
//
//
// Example 1:
//
//
//Input: board =
//[["5","3",".",".","7",".",".",".","."]
//,["6",".",".","1","9","5",".",".","."]
//,[".","9","8",".",".",".",".","6","."]
//,["8",".",".",".","6",".",".",".","3"]
//,["4",".",".","8",".","3",".",".","1"]
//,["7",".",".",".","2",".",".",".","6"]
//,[".","6",".",".",".",".","2","8","."]
//,[".",".",".","4","1","9",".",".","5"]
//,[".",".",".",".","8",".",".","7","9"]]
//Output: true
//
//
// Example 2:
//
//
//Input: board =
//[["8","3",".",".","7",".",".",".","."]
//,["6",".",".","1","9","5",".",".","."]
//,[".","9","8",".",".",".",".","6","."]
//,["8",".",".",".","6",".",".",".","3"]
//,["4",".",".","8",".","3",".",".","1"]
//,["7",".",".",".","2",".",".",".","6"]
//,[".","6",".",".",".",".","2","8","."]
//,[".",".",".","4","1","9",".",".","5"]
//,[".",".",".",".","8",".",".","7","9"]]
//Output: false
//Explanation: Same as Example 1, except with the 5 in the top left corner
//being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is
//invalid.
//
//
//
// Constraints:
//
//
// board.length == 9
// board[i].length == 9
// board[i][j] is a digit 1-9 or '.'.
//
//
// Related Topics Array Hash Table Matrix 👍 9068 👎 946

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = board.len();
        // 校验每行中的数字是否重复
        for i in 0..len {
            if !Solution::is_row_valid(&board[i]) {
                return false;
            }
        }

        // 校验每列中的数字是否重复
        for i in 0..len {
            let mut map = HashMap::new();
            for j in 0..len {
                if !Solution::is_column_valid(board[j][i], &mut map) {
                    return false;
                }
            }
        }

        // 校验sub-boxes中的数字是否重复
        for i in 0..len {
            for j in 0..len {
                if i % 3 == 0 && j % 3 == 0 {
                    if !Solution::is_subboxes_valid(i, j, &board) {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn is_row_valid(row: &[char]) -> bool {
        let mut counter = HashMap::new();
        for &cell in row {
            if !Solution::is_cell_count_valid(cell, &mut counter) {
                return false;
            }
        }
        true
    }

    fn is_column_valid(cell: char, counter: &mut HashMap<char, i32>) -> bool {
        Solution::is_cell_count_valid(cell, counter)
    }

    fn is_subboxes_valid(r_idx: usize, c_idx: usize, board: &[Vec<char>]) -> bool {
        let mut counter = HashMap::new();
        for i in 0..3 {
            for j in 0..3 {
                let cell = board[r_idx + i][c_idx + j];
                if !Solution::is_cell_count_valid(cell, &mut counter) {
                    return false;
                }
            }
        }

        true
    }

    fn is_cell_count_valid(cell: char, counter: &mut HashMap<char, i32>) -> bool {
        if cell == '.' {
            return true;
        }
        counter.entry(cell).and_modify(|k| *k += 1).or_insert(1);
        if counter[&cell] >= 2 {
            return false;
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
