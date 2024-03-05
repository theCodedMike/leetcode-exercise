//Write a program to solve a Sudoku puzzle by filling the empty cells.
//
// A sudoku solution must satisfy all of the following rules:
//
//
// Each of the digits 1-9 must occur exactly once in each row.
// Each of the digits 1-9 must occur exactly once in each column.
// Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-
//boxes of the grid.
//
//
// The '.' character indicates empty cells.
//
//
// Example 1:
//
//
//Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5
//",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".
//",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".
//","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5
//"],[".",".",".",".","8",".",".","7","9"]]
//Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4
//","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3
//"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],[
//"9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3",
//"4","5","2","8","6","1","7","9"]]
//Explanation: The input board is shown above and the only valid solution is
//shown below:
//
//
//
//
//
// Constraints:
//
//
// board.length == 9
// board[i].length == 9
// board[i][j] is a digit or '.'.
// It is guaranteed that the input board has only one solution.
//
//
// Related Topics Array Hash Table Backtracking Matrix 👍 8267 👎 216

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::backtracking(board)

        //Self::bit_operation_optimization(board)
    }

    fn backtracking(board: &mut Vec<Vec<char>>) {
        let mut row = vec![vec![false; 9]; 9];
        let mut col = vec![vec![false; 9]; 9];
        let mut sub_boxes = vec![vec![vec![false; 3]; 3]; 9];
        let mut spaces = vec![];
        let mut valid = false;

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    spaces.push((i, j));
                } else {
                    let idx = (board[i][j] as u8 - b'1') as usize;
                    (row[i][idx], col[j][idx], sub_boxes[idx][i / 3][j / 3]) = (true, true, true);
                }
            }
        }

        const DFS: fn(
            &mut Vec<Vec<char>>,
            usize,
            &Vec<(usize, usize)>,
            &mut Vec<Vec<bool>>,
            &mut Vec<Vec<bool>>,
            &mut Vec<Vec<Vec<bool>>>,
            &mut bool,
        ) = |board, pos, spaces, row, col, sub_boxes, valid| {
            if pos == spaces.len() {
                *valid = true;
                return;
            }

            let (i, j) = spaces[pos];
            for idx in 0..9 {
                if *valid {
                    break;
                }
                if !row[i][idx] && !col[j][idx] && !sub_boxes[idx][i / 3][j / 3] {
                    (row[i][idx], col[j][idx], sub_boxes[idx][i / 3][j / 3]) = (true, true, true);
                    board[i][j] = (b'1' + (idx as u8)) as char;

                    DFS(board, pos + 1, spaces, row, col, sub_boxes, valid);

                    (row[i][idx], col[j][idx], sub_boxes[idx][i / 3][j / 3]) =
                        (false, false, false);
                }
            }
        };

        DFS(
            board,
            0,
            &spaces,
            &mut row,
            &mut col,
            &mut sub_boxes,
            &mut valid,
        );
    }

    fn bit_operation_optimization(board: &mut Vec<Vec<char>>) {
        let mut row = vec![0; 9];
        let mut col = vec![0; 9];
        let mut sub_boxes = vec![vec![0; 3]; 3];
        let mut spaces = vec![];
        let mut valid = false;

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    spaces.push((i, j));
                } else {
                    let digit = (board[i][j] as u8 - b'1') as usize;
                    row[i] ^= 1 << digit;
                    col[j] ^= 1 << digit;
                    sub_boxes[i / 3][j / 3] ^= 1 << digit;
                }
            }
        }

        const DFS: fn(
            &mut Vec<Vec<char>>,
            usize,
            &Vec<(usize, usize)>,
            &mut Vec<i32>,
            &mut Vec<i32>,
            &mut Vec<Vec<i32>>,
            &mut bool,
        ) = |board, pos, spaces, row, col, sub_boxes, valid| {
            if pos == spaces.len() {
                *valid = true;
                return;
            }

            let (i, j) = spaces[pos];
            let mut mask = !(row[i] | col[j] | sub_boxes[i / 3][j / 3]) & 0x1ff;
            while mask != 0 && !*valid {
                let digit = (mask & (-mask)).trailing_zeros();
                row[i] ^= 1 << digit;
                col[j] ^= 1 << digit;
                sub_boxes[i / 3][j / 3] ^= 1 << digit;
                board[i][j] = (digit as u8 + b'1') as char;

                DFS(board, pos + 1, spaces, row, col, sub_boxes, valid);

                row[i] ^= 1 << digit;
                col[j] ^= 1 << digit;
                sub_boxes[i / 3][j / 3] ^= 1 << digit;

                mask &= mask - 1;
            }
        };

        DFS(
            board,
            0,
            &spaces,
            &mut row,
            &mut col,
            &mut sub_boxes,
            &mut valid,
        );
    }
}
//leetcode submit region end(Prohibit modification and deletion)
