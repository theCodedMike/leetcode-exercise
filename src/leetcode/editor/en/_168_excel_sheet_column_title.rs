//Given an integer columnNumber, return its corresponding column title as it
//appears in an Excel sheet.
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
//Input: columnNumber = 1
//Output: "A"
//
//
// Example 2:
//
//
//Input: columnNumber = 28
//Output: "AB"
//
//
// Example 3:
//
//
//Input: columnNumber = 701
//Output: "ZY"
//
//
//
// Constraints:
//
//
// 1 <= columnNumber <= 2³¹ - 1
//
//
// Related Topics Math String 👍 5233 👎 732

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut res = vec![];

        loop {
            column_number -= 1;
            let rem = column_number % 26;
            res.push((b'A' + rem as u8) as char);
            column_number /= 26;

            if column_number == 0 {
                break;
            }
        }

        res.into_iter().rev().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
