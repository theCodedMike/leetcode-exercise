//Given a string s, find the length of the longest substring without repeating
//characters.
//
//
// Example 1:
//
//
//Input: s = "abcabcbb"
//Output: 3
//Explanation: The answer is "abc", with the length of 3.
//
//
// Example 2:
//
//
//Input: s = "bbbbb"
//Output: 1
//Explanation: The answer is "b", with the length of 1.
//
//
// Example 3:
//
//
//Input: s = "pwwkew"
//Output: 3
//Explanation: The answer is "wke", with the length of 3.
//Notice that the answer must be a substring, "pwke" is a subsequence and not a
//substring.
//
//
//
// Constraints:
//
//
// 0 <= s.length <= 5 * 10⁴
// s consists of English letters, digits, symbols and spaces.
//
//
// Related Topics Hash Table String Sliding Window 👍 34340 👎 1507
#![allow(dead_code)]
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut map = HashMap::new();

        let mut i = 0;
        for (j, ch) in s.chars().enumerate() {
            if map.get(&ch).is_some() {
                i = max(map[&ch], i);
            }
            res = max(res, j - i + 1);
            map.insert(ch, j + 1);
        }

        res as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)
