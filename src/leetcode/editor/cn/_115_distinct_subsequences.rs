//Given two strings s and t, return the number of distinct subsequences of s
//which equals t.
//
// The test cases are generated so that the answer fits on a 32-bit signed
//integer.
//
//
// Example 1:
//
//
//Input: s = "rabbbit", t = "rabbit"
//Output: 3
//Explanation:
//As shown below, there are 3 ways you can generate "rabbit" from s.
//rabbbit
//rabbbit
//rabbbit
//
//
// Example 2:
//
//
//Input: s = "babgbag", t = "bag"
//Output: 5
//Explanation:
//As shown below, there are 5 ways you can generate "bag" from s.
//babgbag
//babgbag
//babgbag
//babgbag
//babgbag
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 1000
// s and t consist of English letters.
//
//
// Related Topics String Dynamic Programming 👍 6044 👎 221

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        // todo!
        (s.len() + t.len()) as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
