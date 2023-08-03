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

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let m = s.len();
        let n = t.len();

        let res = "".to_string();
        if m < n {
            return res;
        }

        // Time Limit Exceeded, 需要优化 todo!
        let s_bytes = s.as_bytes();
        let t_map = Solution::slice_to_map(t.as_bytes());

        for size in n..=m {
            for window in s_bytes.windows(size) {
                let map = Solution::slice_to_map(window);
                if Solution::contains_map(&map, &t_map) {
                    return String::from_utf8_lossy(window).to_string();
                }
            }
        }

        res
    }

    fn slice_to_map(arr: &[u8]) -> HashMap<&u8, u16> {
        let mut map = HashMap::new();
        for elem in arr {
            map.entry(elem).and_modify(|v| *v += 1).or_insert(1);
        }
        map
    }

    fn contains_map(first: &HashMap<&u8, u16>, second: &HashMap<&u8, u16>) -> bool {
        for (&key, val) in second {
            if !first.contains_key(key) {
                return false;
            }
            if first[key] < *val {
                return false;
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
