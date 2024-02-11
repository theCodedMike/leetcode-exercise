//The n-queens puzzle is the problem of placing n queens on an n x n chessboard
//such that no two queens attack each other.
//
// Given an integer n, return all distinct solutions to the n-queens puzzle.
//You may return the answer in any order.
//
// Each solution contains a distinct board configuration of the n-queens'
//placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.
//
//
// Example 1:
//
//
//Input: n = 4
//Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
//Explanation: There exist two distinct solutions to the 4-queens puzzle as
//shown above
//
//
// Example 2:
//
//
//Input: n = 1
//Output: [["Q"]]
//
//
//
// Constraints:
//
//
// 1 <= n <= 9
//
//
// Related Topics 数组 回溯 👍 2024 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        Self::backtracking(n)
    }

    fn backtracking(n: i32) -> Vec<Vec<String>> {
        const DFS: fn(i32, i32, &mut Vec<(i32, i32)>, &mut Vec<Vec<String>>) =
            |row, len, pos, res| {
                if row == len {
                    let ans = pos
                        .iter()
                        .map(|&(_, col)| {
                            (0..len).into_iter().fold(
                                String::with_capacity(len as usize),
                                |mut str, c| {
                                    if c == col {
                                        str.push('Q');
                                    } else {
                                        str.push('.');
                                    }
                                    str
                                },
                            )
                        })
                        .collect::<Vec<_>>();
                    res.push(ans);
                    return;
                }

                for col in 0..len {
                    if pos.iter().any(|&(r, c)| {
                        // Same column
                        if col == c {
                            return true;
                        }

                        let slope = (row - r) as f64 / (col - c) as f64;
                        // Same diagonal
                        slope == 1.0 || slope == -1.0
                    }) {
                        continue;
                    }
                    pos.push((row, col));
                    DFS(row + 1, len, pos, res);
                    pos.pop();
                }
            };
        let mut res = vec![];

        DFS(0, n, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
