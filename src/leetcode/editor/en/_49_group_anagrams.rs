//Given an array of strings strs, group the anagrams together. You can return
//the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a
//different word or phrase, typically using all the original letters exactly once.
//
//
// Example 1:
// Input: strs = ["eat","tea","tan","ate","nat","bat"]
//Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//
// Example 2:
// Input: strs = [""]
//Output: [[""]]
//
// Example 3:
// Input: strs = ["a"]
//Output: [["a"]]
//
//
// Constraints:
//
//
// 1 <= strs.length <= 10⁴
// 0 <= strs[i].length <= 100
// strs[i] consists of lowercase English letters.
//
//
// Related Topics Array Hash Table String Sorting 👍 16137 👎 457

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::BTreeMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = BTreeMap::new();
        for str in strs {
            let mut bytes = str.as_bytes().iter().map(|b| *b).collect::<Vec<_>>();
            bytes.sort_unstable();
            let sorted_str = String::from_utf8(bytes).unwrap();
            match map.get_mut(&sorted_str) {
                None => {
                    map.insert(sorted_str, vec![str]);
                }
                Some(v) => v.push(str),
            }
        }

        map.into_values().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
