//The n-queens puzzle is the problem of placing n queens on an n x n chessboard
//such that no two queens attack each other.
//
// Given an integer n, return the number of distinct solutions to the n-queens
//puzzle.
//
//
// Example 1:
//
//
//Input: n = 4
//Output: 2
//Explanation: There are two distinct solutions to the 4-queens puzzle as shown.
//
//
//
// Example 2:
//
//
//Input: n = 1
//Output: 1
//
//
//
// Constraints:
//
//
// 1 <= n <= 9
//
//
// Related Topics 回溯 👍 508 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        Self::backtracking(n)
    }

    fn backtracking(n: i32) -> i32 {
        const DFS: fn(i32, i32, &mut Vec<(i32, i32)>, &mut i32) = |row, len, pos, total| {
            if row == len {
                *total += 1;
                return;
            }

            for col in 0..len {
                if pos.iter().any(|&(r, c)| {
                    // same column
                    if c == col {
                        return true;
                    }

                    let slope = (row - r) as f64 / (col - c) as f64;
                    // same diagonal
                    slope == 1.0 || slope == -1.0
                }) {
                    continue;
                }
                pos.push((row, col));
                DFS(row + 1, len, pos, total);
                pos.pop();
            }
        };
        let mut res = 0;

        DFS(0, n, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
