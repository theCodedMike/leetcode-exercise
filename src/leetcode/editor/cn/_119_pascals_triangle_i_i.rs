//Given an integer rowIndex, return the rowIndexᵗʰ (0-indexed) row of the
//Pascal's triangle.
//
// In Pascal's triangle, each number is the sum of the two numbers directly
//above it as shown:
//
//
// Example 1:
// Input: rowIndex = 3
//Output: [1,3,3,1]
//
// Example 2:
// Input: rowIndex = 0
//Output: [1]
//
// Example 3:
// Input: rowIndex = 1
//Output: [1,1]
//
//
// Constraints:
//
//
// 0 <= rowIndex <= 33
//
//
//
// Follow up: Could you optimize your algorithm to use only O(rowIndex) extra
//space?
//
// Related Topics Array Dynamic Programming 👍 4030 👎 308

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut prev = 0_u64;
        (0..=row_index)
            .map(|idx| {
                if idx == 0 || idx == row_index {
                    prev = 1;
                } else {
                    prev = prev * (row_index + 1 - idx) as u64 / idx as u64;
                }
                prev as i32
            })
            .collect::<Vec<_>>()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
