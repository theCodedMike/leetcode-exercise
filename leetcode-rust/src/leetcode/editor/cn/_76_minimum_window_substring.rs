//Given two strings s and t of lengths m and n respectively, return the minimum
//window substring of s such that every character in t (including duplicates) is
//included in the window. If there is no such substring, return the empty string
//"".
//
// The testcases will be generated such that the answer is unique.
//
//
// Example 1:
//
//
//Input: s = "ADOBECODEBANC", t = "ABC"
//Output: "BANC"
//Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C'
//from string t.
//
//
// Example 2:
//
//
//Input: s = "a", t = "a"
//Output: "a"
//Explanation: The entire string s is the minimum window.
//
//
// Example 3:
//
//
//Input: s = "a", t = "aa"
//Output: ""
//Explanation: Both 'a's from t must be included in the window.
//Since the largest window of s only has one 'a', return empty string.
//
//
//
// Constraints:
//
//
// m == s.length
// n == t.length
// 1 <= m, n <= 10⁵
// s and t consist of uppercase and lowercase English letters.
//
//
//
// Follow up: Could you find an algorithm that runs in O(m + n) time?
//
// Related Topics Hash Table String Sliding Window 👍 15760 👎 652

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let t_map = t.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            map
        });
        let (mut left, mut left_when_min, mut w_len) = (0, 0, usize::MAX);
        let mut s_map = HashMap::new();

        for (right, c) in s.chars().enumerate() {
            s_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            while Self::s_map_contains_t_map(&s_map, &t_map) {
                if w_len > (right - left + 1) {
                    w_len = right - left + 1;
                    left_when_min = left;
                };
                if let Some(v) = s_map.get_mut(&s.chars().nth(left).unwrap()) {
                    *v -= 1;
                }
                left += 1;
            }
        }

        if w_len == usize::MAX {
            "".to_string()
        } else {
            s[left_when_min..left_when_min + w_len].to_string()
        }
    }

    fn s_map_contains_t_map(s_map: &HashMap<char, i32>, t_map: &HashMap<char, i32>) -> bool {
        t_map
            .into_iter()
            .all(|(t_key, &t_val)| s_map.get(t_key).map_or(false, |&s_val| s_val >= t_val))
    }
}
//leetcode submit region end(Prohibit modification and deletion)
