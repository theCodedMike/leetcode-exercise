//Given a positive integer n, generate an n x n matrix filled with elements
//from 1 to n² in spiral order.
//
//
// Example 1:
//
//
//Input: n = 3
//Output: [[1,2,3],[8,9,4],[7,6,5]]
//
//
// Example 2:
//
//
//Input: n = 1
//Output: [[1]]
//
//
//
// Constraints:
//
//
// 1 <= n <= 20
//
//
// Related Topics Array Matrix Simulation 👍 5843 👎 237

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        //Self::simulate(n)
        Self::simulate_by_layer(n)
    }

    fn simulate(n: i32) -> Vec<Vec<i32>> {
        let total_len = n * n;
        let mut res = vec![vec![0; n as usize]; n as usize];
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0;
        let mut col = 0_i32;
        let mut row = 0_i32;

        for i in 1..=total_len {
            res[row as usize][col as usize] = i;
            let next_row = row + directions[dir_idx].0;
            let next_col = col + directions[dir_idx].1;
            if next_row < 0
                || next_row >= n
                || next_col < 0
                || next_col >= n
                || res[next_row as usize][next_col as usize] != 0
            {
                dir_idx = (dir_idx + 1) % 4;
            }

            row += directions[dir_idx].0;
            col += directions[dir_idx].1;
        }

        res
    }

    fn simulate_by_layer(n: i32) -> Vec<Vec<i32>> {
        let mut left = 0;
        let mut right = n - 1;
        let mut top = 0;
        let mut bottom = n - 1;
        let mut res = vec![vec![0; n as usize]; n as usize];
        let mut val = 1;

        while left <= right && top <= bottom {
            // left(top) -> right(top)
            for col in left..=right {
                res[top as usize][col as usize] = val;
                val += 1;
            }
            // right(top)
            //     ↓
            // right(bottom)
            for row in top + 1..=bottom {
                res[row as usize][right as usize] = val;
                val += 1;
            }
            if left < right && top < bottom {
                // left(bottom) <- right(bottom)
                for col in (left + 1..=right - 1).rev() {
                    res[bottom as usize][col as usize] = val;
                    val += 1;
                }
                // left(top)
                //     ↑
                // left(bottom)
                for row in (top + 1..=bottom).rev() {
                    res[row as usize][left as usize] = val;
                    val += 1;
                }
            }
            left += 1;
            right -= 1;
            top += 1;
            bottom -= 1;
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
