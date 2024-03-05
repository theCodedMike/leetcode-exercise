//Given a triangle array, return the minimum path sum from top to bottom.
//
// For each step, you may move to an adjacent number of the row below. More
//formally, if you are on index i on the current row, you may move to either index i
//or index i + 1 on the next row.
//
//
// Example 1:
//
//
//Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
//Output: 11
//Explanation: The triangle looks like:
//   2
//  3 4
// 6 5 7
//4 1 8 3
//The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined
//above).
//
//
// Example 2:
//
//
//Input: triangle = [[-10]]
//Output: -10
//
//
//
// Constraints:
//
//
// 1 <= triangle.length <= 200
// triangle[0].length == 1
// triangle[i].length == triangle[i - 1].length + 1
// -10⁴ <= triangle[i][j] <= 10⁴
//
//
//
//Follow up: Could you do this using only
//O(n) extra space, where
//n is the total number of rows in the triangle?
//
// Related Topics Array Dynamic Programming 👍 8795 👎 508

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // todo!
        //Self::dp_helper(triangle)
        Self::dp_optimize(triangle)
    }

    /// time: O(n^2)
    ///
    /// space: O(n^2)
    pub fn dp_helper(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        let mut f = vec![vec![0; len]; len];
        f[0][0] = triangle[0][0];

        for i in 1..len {
            f[i][0] = f[i - 1][0] + triangle[i][0];
            for j in 1..i {
                f[i][j] = std::cmp::min(f[i - 1][j - 1], f[i - 1][j]) + triangle[i][j];
            }
            f[i][i] = f[i - 1][i - 1] + triangle[i][i];
        }

        *f[len - 1].iter().min().unwrap()
    }

    /// time: O(n^2)
    ///
    /// space: O(n)
    pub fn dp_optimize(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        let mut f = vec![0; len];
        f[0] = triangle[0][0];

        for i in 1..len {
            f[i] = f[i - 1] + triangle[i][i];
            for j in (1..i).rev() {
                f[j] = std::cmp::min(f[j - 1], f[j]) + triangle[i][j];
            }
            f[0] += triangle[i][0];
        }

        *f.iter().min().unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
