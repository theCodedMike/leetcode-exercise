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
// Related Topics String Dynamic Programming 👍 27160 👎 1603

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //Self::use_dp(s)
        Self::expand_around_center(s)
    }

    fn use_dp(s: String) -> String {
        let len = s.len();
        if len < 2 {
            return s;
        }
        let mut max_len = 1;
        let mut begin = 0;
        // dp[i][j] 表示 s[i..j] 是否是回文串
        let mut dp = vec![vec![false; len]; len];
        // 初始化：所有长度为 1 的子串都是回文串
        for i in 0..len {
            dp[i][i] = true;
        }

        // 先枚举子串长度
        for l in 2..=len {
            // 枚举左边界，左边界的上限设置可以宽松一些
            for i in 0..len {
                // 由 L 和 i 可以确定右边界，即 j - i + 1 = L 得
                let j = l + i - 1;
                // 如果右边界越界，就可以退出当前循环
                if j >= len {
                    break;
                }

                if s.get(i..i + 1) != s.get(j..j + 1) {
                    dp[i][j] = false;
                } else {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i + 1][j - 1];
                    }
                }

                // 只要 dp[i][L] == true 成立，就表示子串 s[i..L] 是回文，此时记录回文长度和起始位置
                if dp[i][j] && l > max_len {
                    max_len = l;
                    begin = i;
                }
            }
        }

        s.index(begin..begin + max_len).to_string()
    }

    fn expand_around_center(s: String) -> String {
        let len = s.len() as i32;
        if len < 2 {
            return s;
        }

        let expand = |s: &str, mut left: i32, mut right: i32, len: i32| -> i32 {
            while 0 <= left
                && right < len
                && s.get(left as usize..left as usize + 1)
                    == s.get(right as usize..right as usize + 1)
            {
                left -= 1;
                right += 1;
            }

            right - left - 1
        };

        let mut start = 0;
        let mut end = 0;
        for i in 0..len {
            let len1 = expand(&s, i, i, len);
            let len2 = expand(&s, i, i + 1, len);
            let max = std::cmp::max(len1, len2);
            if max > end - start {
                start = i - (max - 1) / 2;
                end = i + max / 2;
            }
        }

        s.index(start as usize..end as usize + 1).to_string()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
