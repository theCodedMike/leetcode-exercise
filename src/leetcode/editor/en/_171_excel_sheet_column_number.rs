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
use std::collections::HashMap;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let map = ('A'..='Z').zip(1..=26).collect::<HashMap<char, i32>>();
        let len = column_title.len() - 1;
        let mut num = 0;

        for (idx, c) in column_title.chars().enumerate() {
            num += 26_i32.pow((len - idx) as u32) * map[&c];
        }

        num
    }
}
//leetcode submit region end(Prohibit modification and deletion)
