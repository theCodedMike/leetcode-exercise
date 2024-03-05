//Given a string columnTitle that represents the column title as appears in an
//Excel sheet, return its corresponding column number.
//
// For example:
//
//
//A -> 1
//B -> 2
//C -> 3
//...
//Z -> 26
//AA -> 27
//AB -> 28
//...
//
//
//
// Example 1:
//
//
//Input: columnTitle = "A"
//Output: 1
//
//
// Example 2:
//
//
//Input: columnTitle = "AB"
//Output: 28
//
//
// Example 3:
//
//
//Input: columnTitle = "ZY"
//Output: 701
//
//
//
// Constraints:
//
//
// 1 <= columnTitle.length <= 7
// columnTitle consists only of uppercase English letters.
// columnTitle is in the range ["A", "FXSHRXW"].
//
//
// Related Topics Math String 👍 4526 👎 347

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let len = column_title.len() - 1;
        let a = b'A';
        column_title.chars().enumerate().fold(0, |acc, (idx, c)| {
            let curr = 26_i32.pow((len - idx) as u32) * (c as u8 - a + 1) as i32;
            acc + curr
        })
    }
}
//leetcode submit region end(Prohibit modification and deletion)
