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
        let mut len = n as usize;
        let mut matrix = vec![vec![0; len]; len];

        let iter_count = (len as f64 / 2.0).ceil() as usize;
        let mut val = 1;
        for i in 0..iter_count {
            Solution::process_outer(i, i, &mut matrix, &mut val, len);
            if len <= 2 {
                if len == 1 {
                    matrix[i][i] = val;
                }
                break;
            }
            len -= 2;
        }

        matrix
    }

    fn process_outer(
        mut x: usize,
        mut y: usize,
        matrix: &mut Vec<Vec<i32>>,
        val: &mut i32,
        outer_len: usize,
    ) {
        // left -> right
        for _ in 1..outer_len {
            matrix[x][y] = *val;
            y += 1;
            *val += 1;
        }
        // top
        //  ↓
        // bottom
        for _ in 1..outer_len {
            matrix[x][y] = *val;
            x += 1;
            *val += 1;
        }
        // left <- right
        for _ in 1..outer_len {
            matrix[x][y] = *val;
            y -= 1;
            *val += 1;
        }
        // top
        //  ↑
        // bottom
        for _ in 1..outer_len {
            matrix[x][y] = *val;
            x -= 1;
            *val += 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
