//Given an input string s and a pattern p, implement regular expression
//matching with support for '.' and '*' where:
//
//
// '.' Matches any single character.
// '*' Matches zero or more of the preceding element.
//
//
// The matching should cover the entire input string (not partial).
//
//
// Example 1:
//
//
//Input: s = "aa", p = "a"
//Output: false
//Explanation: "a" does not match the entire string "aa".
//
//
// Example 2:
//
//
//Input: s = "aa", p = "a*"
//Output: true
//Explanation: '*' means zero or more of the preceding element, 'a'. Therefore,
//by repeating 'a' once, it becomes "aa".
//
//
// Example 3:
//
//
//Input: s = "ab", p = ".*"
//Output: true
//Explanation: ".*" means "zero or more (*) of any character (.)".
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 20
// 1 <= p.length <= 20
// s contains only lowercase English letters.
// p contains only lowercase English letters, '.', and '*'.
// It is guaranteed for each appearance of the character '*', there will be a
//previous valid character to match.
//
//
// Related Topics String Dynamic Programming Recursion 👍 10820 👎 1777

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut res = false;
        todo!();
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
