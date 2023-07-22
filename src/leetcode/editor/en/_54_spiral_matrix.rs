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
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let mut res = Vec::with_capacity(row_len * col_len);

        let iter_count = (std::cmp::min(row_len, col_len) as f64 / 2.0).ceil() as usize;
        let mut rows = row_len;
        let mut cols = col_len;
        for i in 0..iter_count {
            Solution::outer_iter(&matrix, rows, cols, i, i, &mut res);
            if rows <= 2 || cols <= 2 {
                if rows == 1 && cols == 1 {
                    // 对于 1 x 1 的矩阵(即1个元素)，outer_iter无法处理，所以直接push
                    res.push(matrix[i][i]);
                }
                break;
            }
            rows -= 2;
            cols -= 2;
        }

        // 之所以resize，是因为对于下面这种类型，push顺序为 7->9->6->9, 9被push了2次，所以需要resize
        // [[7]
        // ,[9]
        // ,[6]]
        res.resize(row_len * col_len, 0);

        res
    }

    /// 遍历最外层
    fn outer_iter(
        matrix: &Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
        mut x: usize,
        mut y: usize,
        res: &mut Vec<i32>,
    ) {
        // 左 -> 右
        for _ in 1..cols {
            res.push(matrix[x][y]);
            y += 1;
        }

        // 上 -> 下
        for _ in 1..rows {
            res.push(matrix[x][y]);
            x += 1;
        }

        // 右 -> 左
        for _ in 1..cols {
            res.push(matrix[x][y]);
            y -= 1;
        }

        // 下 -> 上
        for _ in 1..rows {
            res.push(matrix[x][y]);
            x -= 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
