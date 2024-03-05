//Given an m x n integer matrix matrix, if an element is 0, set its entire row
//and column to 0's.
//
// You must do it in place.
//
//
// Example 1:
//
//
//Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
//Output: [[1,0,1],[0,0,0],[1,0,1]]
//
//
// Example 2:
//
//
//Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
//Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[0].length
// 1 <= m, n <= 200
// -2³¹ <= matrix[i][j] <= 2³¹ - 1
//
//
//
// Follow up:
//
//
// A straightforward solution using O(mn) space is probably a bad idea.
// A simple improvement uses O(m + n) space, but still not the best solution.
// Could you devise a constant space solution?
//
//
// Related Topics Array Hash Table Matrix 👍 12484 👎 639

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let row_len = matrix.len();
        let col_len = matrix[0].len();
        let mut row_cache = HashSet::with_capacity(row_len);
        let mut col_cache = HashSet::with_capacity(col_len);

        // statistics
        for i in 0..row_len {
            let mut row_has_zero = false;

            for j in 0..col_len {
                if matrix[i][j] == 0 {
                    row_has_zero = true;
                    col_cache.insert(j);
                }
            }

            if row_has_zero {
                row_cache.insert(i);
            }
        }

        // set row to zero
        for i in 0..row_len {
            if row_cache.contains(&i) {
                for j in 0..col_len {
                    matrix[i][j] = 0;
                }
            }
        }

        // set col to zero
        for j in 0..col_len {
            if col_cache.contains(&j) {
                for i in 0..row_len {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
