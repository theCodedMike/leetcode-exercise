//You are given an n x n 2D matrix representing an image, rotate the image by 90
// degrees (clockwise).
//
// You have to rotate the image in-place, which means you have to modify the
//input 2D matrix directly. DO NOT allocate another 2D matrix and do the rotation.
//
//
// Example 1:
//
//
//Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
//Output: [[7,4,1],[8,5,2],[9,6,3]]
//
//
// Example 2:
//
//
//Input: matrix = [[5,1,9,11],[2,4,8,10],[13,3,6,7],[15,14,12,16]]
//Output: [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]
//
//
//
// Constraints:
//
//
// n == matrix.length == matrix[i].length
// 1 <= n <= 20
// -1000 <= matrix[i][j] <= 1000
//
//
// Related Topics Array Math Matrix 👍 15294 👎 674

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        // 对角线交换
        for i in 0..len {
            for j in i..len {
                if i != j {
                    Solution::diagonal_swap(matrix, i, j);
                }
            }
        }
        // 左右交换
        for i in 0..len {
            for j in 0..(len / 2) {
                matrix[i].swap(j, len - 1 - j);
            }
        }
    }

    fn diagonal_swap(matrix: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        let temp = matrix[i][j];
        matrix[i][j] = matrix[j][i];
        matrix[j][i] = temp;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
