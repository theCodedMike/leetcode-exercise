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
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x != 0 && x % 10 == 0) {
            return false;
        }

        let mut reverted_num = 0;
        while x > reverted_num {
            reverted_num = reverted_num * 10 + x % 10;
            x /= 10;
        }

        x == reverted_num || x == reverted_num / 10
    }
}
//leetcode submit region end(Prohibit modification and deletion)
