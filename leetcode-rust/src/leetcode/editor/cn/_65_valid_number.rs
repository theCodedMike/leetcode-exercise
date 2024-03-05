//A valid number can be split up into these components (in order):
//
//
// A decimal number or an integer.
// (Optional) An 'e' or 'E', followed by an integer.
//
//
// A decimal number can be split up into these components (in order):
//
//
// (Optional) A sign character (either '+' or '-').
// One of the following formats:
//
// One or more digits, followed by a dot '.'.
// One or more digits, followed by a dot '.', followed by one or more digits.
// A dot '.', followed by one or more digits.
//
//
//
// An integer can be split up into these components (in order):
//
//
// (Optional) A sign character (either '+' or '-').
// One or more digits.
//
//
// For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.1
//4", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"],
//while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "-
//-6", "-+3", "95a54e53"].
//
// Given a string s, return true if s is a valid number.
//
//
// Example 1:
//
//
//Input: s = "0"
//Output: true
//
//
// Example 2:
//
//
//Input: s = "e"
//Output: false
//
//
// Example 3:
//
//
//Input: s = "."
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 20
// s consists of only English letters (both uppercase and lowercase), digits (0-
//9), plus '+', minus '-', or dot '.'.
//
//
// Related Topics String 👍 984 👎 1632

#![allow(dead_code)]
//    Input             Output
//      0e              false
//      0..             false
//      .1.             false
//      6+1             false
//      2e0              true
//       +              false
//     6e6.5            false

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // 连续出现错误, 比如 "--6" "-+3" "0.." "0eE2"
    fn continuation_error(next: Option<&(usize, char)>, candidates: &str) -> bool {
        match next {
            None => false,
            Some((_, c)) => candidates.contains(*c),
        }
    }

    // 个数错误, 比如 "0.." "0eE2", 因为像 'e'/'E'/'.'这种字符只能出现0/1次
    fn count_error(count: &mut i32) -> bool {
        *count += 1;
        if *count > 1 {
            return true;
        }
        false
    }

    fn determine_prev_char_is_e(s: &str, index: usize) -> bool {
        for (idx, c) in s.chars().enumerate() {
            if idx == (index - 1) {
                return match c {
                    'e' | 'E' => true,
                    _ => false,
                };
            }
        }
        false
    }

    pub fn is_number(s: String) -> bool {
        let mut iter = s.chars().enumerate().peekable();
        let mut has_digits = false;
        let mut dot_count = 0;
        let mut e_count = 0;

        while let Some((idx, c)) = iter.next() {
            match c {
                '0'..='9' => {
                    has_digits = true;
                }
                '+' | '-' => {
                    if Solution::continuation_error(iter.peek(), "+-") {
                        return false;
                    }
                    if !(idx == 0 || Solution::determine_prev_char_is_e(&s, idx)) {
                        // '+'/'-'是符号位，要么出现在首位，要么出现在'e'/'E'的后面
                        return false;
                    }
                    match iter.peek() {
                        // '+'/'-'的后面没有字符了
                        None => return false,
                        Some(_) => {}
                    }
                }
                'e' | 'E' => {
                    if Solution::count_error(&mut e_count) {
                        // 'e'/'E'的个数 > 1
                        return false;
                    }
                    if !has_digits {
                        // 'e'/'E'的前面没有任何数字
                        return false;
                    }
                    match s.get((idx + 1)..) {
                        None => return false, // 'e'/'E'的后面没有出现数字
                        Some(s) => {
                            if s.is_empty()
                                || s.contains(|c| !('0' <= c && c <= '9' || c == '+' || c == '-'))
                            {
                                // 'e'/'E'后面出现的数字中包含dot.或其他字符
                                return false;
                            }
                        }
                    }
                }
                '.' => {
                    if Solution::count_error(&mut dot_count) {
                        // dot.的个数 > 1
                        return false;
                    }
                    if !has_digits
                        && match iter.peek() {
                            None => true,
                            Some((_, c)) => !"0123456789".contains(*c),
                        }
                    {
                        // dot.的前后同时没有数字
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
