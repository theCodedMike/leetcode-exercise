//You are given an m x n integer matrix matrix with the following two
//properties:
//
//
// Each row is sorted in non-decreasing order.
// The first integer of each row is greater than the last integer of the
//previous row.
//
//
// Given an integer target, return true if target is in matrix or false
//otherwise.
//
// You must write a solution in O(log(m * n)) time complexity.
//
//
// Example 1:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
//Output: true
//
//
// Example 2:
//
//
//Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
//Output: false
//
//
//
// Constraints:
//
//
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 100
// -10⁴ <= matrix[i][j], target <= 10⁴
//
//
// Related Topics Array Binary Search Matrix 👍 13179 👎 364

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row_len = matrix.len();
        let col_len = matrix[0].len();

        let mut low = 0;
        let mut high = col_len * row_len;
        let mut mid = 0;
        while low < high {
            mid = (low + high) / 2;
            let (i, j) = Solution::convert_idx(mid, col_len);
            if target > matrix[i][j] {
                low = mid + 1;
            } else if target < matrix[i][j] {
                high = mid;
            } else {
                return true;
            }
        }

        false
    }

    fn convert_idx(idx: usize, col_len: usize) -> (usize, usize) {
        let i = idx / col_len;
        let j = idx % col_len;
        (i, j)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
