//Given a string s, return the longest palindromic substring in s.
//
//
// Example 1:
//
//
//Input: s = "babad"
//Output: "bab"
//Explanation: "aba" is also a valid answer.
//
//
// Example 2:
//
//
//Input: s = "cbbd"
//Output: "bb"
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 1000
// s consist of only digits and English letters.
//
//
// Related Topics String Dynamic Programming 👍 25643 👎 1503

#![allow(dead_code)]

use std::ops::Index;

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let len = s.len();
        if len == 1 {
            return s;
        }

        "".to_string()
    }
}

fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    let half = len / 2;

    let mut i = 0;
    while i < half {
        if s.index(i..i + 1) != s.index((len - 1 - i)..(len - i)) {
            return false;
        }
        i += 1;
    }

    true
}

//leetcode submit region end(Prohibit modification and deletion)
