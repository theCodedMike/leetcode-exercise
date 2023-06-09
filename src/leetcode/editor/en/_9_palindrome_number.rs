//Given an integer x, return true if x is a palindrome, and false otherwise.
//
//
// Example 1:
//
//
//Input: x = 121
//Output: true
//Explanation: 121 reads as 121 from left to right and from right to left.
//
//
// Example 2:
//
//
//Input: x = -121
//Output: false
//Explanation: From left to right, it reads -121. From right to left, it
//becomes 121-. Therefore it is not a palindrome.
//
//
// Example 3:
//
//
//Input: x = 10
//Output: false
//Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
//
//
// Constraints:
//
//
// -2³¹ <= x <= 2³¹ - 1
//
//
//
//Follow up: Could you solve it without converting the integer to a string?
//
// Related Topics 数学 👍 2563 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x_str = x.to_string();
        let len = x_str.len();
        let half_len = len / 2;
        for i in 0..half_len {
            if x_str.get(i..i + 1) != x_str.get(len - i - 1..len - i) {
                return false;
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
