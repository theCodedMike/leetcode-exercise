//Write a function to find the longest common prefix string amongst an array of
//strings.
//
// If there is no common prefix, return an empty string "".
//
//
// Example 1:
//
//
//Input: strs = ["flower","flow","flight"]
//Output: "fl"
//
//
// Example 2:
//
//
//Input: strs = ["dog","racecar","car"]
//Output: ""
//Explanation: There is no common prefix among the input strings.
//
//
//
// Constraints:
//
//
// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters.
//
//
// Related Topics String Trie 👍 14171 👎 3991

#![allow(dead_code)]

use std::ops::Index;

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let len = strs.len();
        if len == 0 {
            return "".to_string();
        }
        if len == 1 {
            return strs[0].to_string();
        }

        let first = strs[0].as_str();
        let first_len = first.len();
        if first_len == 0 {
            return "".to_string();
        }

        let mut i = 1;
        while i <= first_len {
            let prefix = first.get(..i).unwrap();

            for j in 1..len {
                let str = strs[j].as_str();
                if str.is_empty() {
                    return "".to_string();
                }

                if !str.starts_with(prefix) {
                    return first.get(..i - 1).unwrap().to_string();
                }
            }

            i += 1;
        }

        first.get(..i - 1).unwrap().to_string()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
