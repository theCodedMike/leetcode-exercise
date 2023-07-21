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
        let mut rows = matrix.len();
        let mut cols = matrix[0].len();

        let mut res = Vec::with_capacity(rows * cols);

        let iter_count = (std::cmp::min(rows, cols) as f64 / 2.0).ceil() as usize;
        for i in 0..iter_count {
            Solution::outer_iter(&matrix, rows, cols, i, i, &mut res);
            if rows <= 2 || cols <= 2 {
                break;
            }
            rows -= 2;
            cols -= 2;
        }

        res
    }

    fn outer_iter(
        matrix: &Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
        mut begin_x: usize,
        mut begin_y: usize,
        res: &mut Vec<i32>,
    ) {
        // 左 -> 右
        for y in 0..cols {
            res.push(matrix[begin_x][begin_y + y]);
        }
        begin_y += cols - 1;

        // 上 -> 下
        begin_x += 1;
        if rows < 2 {
            return;
        }
        for x in 0..(rows - 2) {
            res.push(matrix[begin_x + x][begin_y]);
        }
        begin_x += rows - 3;

        // 右 -> 左
        begin_x += 1;
        for y in 0..cols {
            res.push(matrix[begin_x][begin_y - y]);
        }
        begin_y -= cols - 1;

        // 下 -> 上
        begin_x -= 1;
        if cols <= 1 {
            return;
        }
        for x in 0..(rows - 2) {
            res.push(matrix[begin_x - x][begin_y]);
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
