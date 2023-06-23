//Implement the myAtoi(string s) function, which converts a string to a 32-bit
//signed integer (similar to C/C++'s atoi function).
//
// The algorithm for myAtoi(string s) is as follows:
//
//
// Read in and ignore any leading whitespace.
// Check if the next character (if not already at the end of the string) is '-'
//or '+'. Read this character in if it is either. This determines if the final
//result is negative or positive respectively. Assume the result is positive if
//neither is present.
// Read in next the characters until the next non-digit character or the end of
//the input is reached. The rest of the string is ignored.
// Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If
//no digits were read, then the integer is 0. Change the sign as necessary (from
//step 2).
// If the integer is out of the 32-bit signed integer range [-2³¹, 2³¹ - 1],
//then clamp the integer so that it remains in the range. Specifically, integers
//less than -2³¹ should be clamped to -2³¹, and integers greater than 2³¹ - 1 should
//be clamped to 2³¹ - 1.
// Return the integer as the final result.
//
//
// Note:
//
//
// Only the space character ' ' is considered a whitespace character.
// Do not ignore any characters other than the leading whitespace or the rest
//of the string after the digits.
//
//
//
// Example 1:
//
//
//Input: s = "42"
//Output: 42
//Explanation: The underlined characters are what is read in, the caret is the
//current reader position.
//Step 1: "42" (no characters read because there is no leading whitespace)
//         ^
//Step 2: "42" (no characters read because there is neither a '-' nor '+')
//         ^
//Step 3: "42" ("42" is read in)
//           ^
//The parsed integer is 42.
//Since 42 is in the range [-2³¹, 2³¹ - 1], the final result is 42.
//
//
// Example 2:
//
//
//Input: s = "   -42"
//Output: -42
//Explanation:
//Step 1: "   -42" (leading whitespace is read and ignored)
//            ^
//Step 2: "   -42" ('-' is read, so the result should be negative)
//             ^
//Step 3: "   -42" ("42" is read in)
//               ^
//The parsed integer is -42.
//Since -42 is in the range [-2³¹, 2³¹ - 1], the final result is -42.
//
//
// Example 3:
//
//
//Input: s = "4193 with words"
//Output: 4193
//Explanation:
//Step 1: "4193 with words" (no characters read because there is no leading
//whitespace)
//         ^
//Step 2: "4193 with words" (no characters read because there is neither a '-'
//nor '+')
//         ^
//Step 3: "4193 with words" ("4193" is read in; reading stops because the next
//character is a non-digit)
//             ^
//The parsed integer is 4193.
//Since 4193 is in the range [-2³¹, 2³¹ - 1], the final result is 4193.
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 200
// s consists of English letters (lower-case and upper-case), digits (0-9), ' ',
// '+', '-', and '.'.
//
//
// Related Topics String 👍 3392 👎 10463

#![allow(dead_code)]

pub struct Solution;

/// 测试用例: "+-12"
/// 期望结果: 0
///
/// 测试用例: "00000-42a1234"
/// 期望结果: 0
///
/// 测试用例: "   +0 123"
/// 期望结果: 0
//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut res = 0_i64;
        let mut temp = 0_i64;
        let i32_max = i32::MAX as i64;
        let mut positive = true; // 正数或负数
        let mut sign_count = 0; //'+'/'-'符号的个数，只能有1个
        let mut detect_digit = false; //检测到数字
        let mut detect_valid_char = false; // 检测到有效字符，即"+-0123456789"

        for x in s.chars() {
            if x == ' ' {
                if detect_valid_char {
                    break;
                }

                continue;
            } else {
                detect_valid_char = true;

                match x {
                    '-' | '+' => {
                        if detect_digit {
                            // 说明'+'/'-'出现在数字的后面
                            break;
                        }
                        if x == '-' {
                            positive = false;
                        }
                        sign_count += 1;
                        if sign_count >= 2 {
                            res = 0;
                            break;
                        }
                    }
                    '0'..='9' => {
                        detect_digit = true;

                        temp = res * 10 + convert_char_to_digit(x);

                        if positive && temp > i32_max {
                            res = i32_max;
                            break;
                        }
                        if !positive && temp > (i32_max + 1) {
                            res = i32_max + 1;
                            break;
                        }

                        res = temp;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        if !positive {
            res = (0 - res);
        }

        res as i32
    }
}

pub fn convert_char_to_digit(c: char) -> i64 {
    match c {
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0,
    }
}
//leetcode submit region end(Prohibit modification and deletion)
