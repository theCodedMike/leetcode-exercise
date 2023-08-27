//A phrase is a palindrome if, after converting all uppercase letters into
//lowercase letters and removing all non-alphanumeric characters, it reads the same
//forward and backward. Alphanumeric characters include letters and numbers.
//
// Given a string s, return true if it is a palindrome, or false otherwise.
//
//
// Example 1:
//
//
//Input: s = "A man, a plan, a canal: Panama"
//Output: true
//Explanation: "amanaplanacanalpanama" is a palindrome.
//
//
// Example 2:
//
//
//Input: s = "race a car"
//Output: false
//Explanation: "raceacar" is not a palindrome.
//
//
// Example 3:
//
//
//Input: s = " "
//Output: true
//Explanation: s is an empty string "" after removing non-alphanumeric
//characters.
//Since an empty string reads the same forward and backward, it is a palindrome.
//
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 2 * 10⁵
// s consists only of printable ASCII characters.
//
//
// Related Topics Two Pointers String 👍 7912 👎 7714

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        //Self::filter_then_compare(&s)
        Self::directly_compare(s.as_bytes())
    }

    pub fn filter_then_compare(ori: &str) -> bool {
        let str = ori
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| {
                if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect::<String>();
        let len = str.len();
        if len == 0 {
            return true;
        }

        let mut start = 0;
        let mut end = len - 1;
        while start < end {
            if str.index(start..start + 1) != str.index(end..end + 1) {
                return false;
            }
            start += 1;
            end -= 1;
        }

        true
    }

    pub fn directly_compare(ori: &[u8]) -> bool {
        let len = ori.len();
        let mut start = 0;
        let mut end = len - 1;
        while start < end {
            let mut s_char = *ori.index(start);
            let mut e_char = *ori.index(end);
            match (
                s_char.is_ascii_alphanumeric(),
                e_char.is_ascii_alphanumeric(),
            ) {
                (false, false) => {
                    start += 1;
                    end -= 1;
                }
                (true, false) => {
                    end -= 1;
                }
                (false, true) => {
                    start += 1;
                }
                (true, true) => {
                    if s_char.is_ascii_uppercase() {
                        s_char = s_char.to_ascii_lowercase();
                    }
                    if e_char.is_ascii_uppercase() {
                        e_char = e_char.to_ascii_lowercase();
                    }
                    if s_char != e_char {
                        return false;
                    }
                    start += 1;
                    end -= 1;
                }
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
