//Given an m x n matrix, return all elements of the matrix in spiral order.
//
//
// Example 1:
//
//
//Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [1,2,3,6,9,8,7,4,5]
//
//
// Example 2:
//
//
//Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
//Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 10
// -100 <= matrix[i][j] <= 100
//
//
// Related Topics Array Matrix Simulation 👍 12764 👎 1135

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        //Self::simulate(matrix)
        Self::simulate_by_layer(matrix)
    }

    /// 时间复杂度：O(row * col)
    ///
    /// 空间复杂度：O(1)
    fn simulate(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len() as i32;
        let col = matrix[0].len() as i32;
        let total_len = (row * col) as usize;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir_idx = 0;
        let mut i = 0_i32;
        let mut j = 0_i32;
        let mut res = Vec::with_capacity(total_len);
        for _ in 1..=total_len {
            res.push(matrix[i as usize][j as usize]);
            matrix[i as usize][j as usize] = i32::MIN;

            let next_i = i + directions[dir_idx].0;
            let next_j = j + directions[dir_idx].1;
            if next_i < 0
                || next_i >= row
                || next_j < 0
                || next_j >= col
                || matrix[next_i as usize][next_j as usize] == i32::MIN
            {
                dir_idx = (dir_idx + 1) % 4;
            }

            i += directions[dir_idx].0;
            j += directions[dir_idx].1;
        }

        res
    }

    /// 时间复杂度：O(row * col)
    ///
    /// 空间复杂度：O(1)
    fn simulate_by_layer(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let row = matrix.len();
        let col = matrix[0].len();
        let total_len = row * col;

        let mut left = 0_i32;
        let mut right = (col - 1) as i32;
        let mut top = 0_i32;
        let mut bottom = (row - 1) as i32;
        let mut res = Vec::with_capacity(total_len);
        while left <= right && top <= bottom {
            // left(top) -> right(top)
            for j in left..=right {
                res.push(matrix[top as usize][j as usize]);
            }
            // right(top)
            //   ↓
            // right(bottom)
            for i in top + 1..=bottom {
                res.push(matrix[i as usize][right as usize]);
            }

            if left < right && top < bottom {
                // left(bottom) <- right(bottom)
                for j in (left + 1..=right - 1).rev() {
                    res.push(matrix[bottom as usize][j as usize]);
                }
                // left(top)
                //   ↑
                // left(bottom)
                for i in (top + 1..=bottom).rev() {
                    res.push(matrix[i as usize][left as usize]);
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
