//Given an m x n matrix board containing 'X' and 'O', capture all regions that
//are 4-directionally surrounded by 'X'.
//
// A region is captured by flipping all 'O's into 'X's in that surrounded
//region.
//
//
// Example 1:
//
//
//Input: board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X",
//"O","X","X"]]
//Output: [["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X",
//"X"]]
//Explanation: Notice that an 'O' should not be flipped if:
//- It is on the border, or
//- It is adjacent to an 'O' that should not be flipped.
//The bottom 'O' is on the border, so it is not flipped.
//The other three 'O' form a surrounded region, so they are flipped.
//
//
// Example 2:
//
//
//Input: board = [["X"]]
//Output: [["X"]]
//
//
//
// Constraints:
//
//
// m == board.length
// n == board[i].length
// 1 <= m, n <= 200
// board[i][j] is 'X' or 'O'.
//
//
// Related Topics Array Depth-First Search Breadth-First Search Union Find
//Matrix 👍 7789 👎 1625

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let rows = board.len();
        let cols = board[0].len();

        // todo!
        for i in 0..rows {
            for j in 0..cols {
                if board[i][j] == 'O' {
                    if Self::horizontal_direction_surrounded_by_x(cols, j, &board[i])
                        && Self::vertical_direction_surrounded_by_x(rows, i, j, board)
                    {
                        board[i][j] = 'X';
                    }
                }
            }
        }
    }

    fn horizontal_direction_surrounded_by_x(cols: usize, col_idx: usize, row: &Vec<char>) -> bool {
        let mut prev_x_exist = false;
        for j in (0..col_idx).rev() {
            if row[j] == 'X' {
                prev_x_exist = true;
                break;
            }
        }

        let mut next_x_exist = false;
        for j in col_idx + 1..cols {
            if row[j] == 'X' {
                next_x_exist = true;
                break;
            }
        }

        prev_x_exist && next_x_exist
    }

    fn vertical_direction_surrounded_by_x(
        rows: usize,
        row_idx: usize,
        col_idx: usize,
        board: &mut Vec<Vec<char>>,
    ) -> bool {
        let mut top_x_exist = false;
        for i in (0..row_idx).rev() {
            if board[i][col_idx] == 'X' {
                top_x_exist = true;
                break;
            }
        }

        let mut down_x_exist = false;
        for i in row_idx + 1..rows {
            if board[i][col_idx] == 'X' {
                down_x_exist = true;
                break;
            }
        }

        top_x_exist && down_x_exist
    }
}
//leetcode submit region end(Prohibit modification and deletion)
