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
use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        Self::sorting(strs)
        //Self::counting(strs)
    }

    fn sorting(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::with_capacity(strs.len());
        for str in strs {
            let mut key = str.clone().into_bytes();
            key.sort_unstable();
            match map.get_mut(&key) {
                None => {
                    map.insert(key, vec![str]);
                }
                Some(v) => {
                    v.push(str);
                }
            }
        }
        map.into_values().collect()
    }

    fn counting(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::with_capacity(strs.len());
        let a_u8 = b'a';
        let mut counter = [0; 26];

        for str in strs {
            for c in str.chars() {
                counter[(c as u8 - a_u8) as usize] += 1;
            }
            let mut key = String::with_capacity(str.len());
            for i in 0..26 {
                if counter[i] != 0 {
                    key.push_str(i.to_string().as_str());
                    counter[i] = 0;
                }
            }
            match map.get_mut(&key) {
                None => {
                    map.insert(key, vec![str]);
                }
                Some(v) => {
                    v.push(str);
                }
            }
        }

        map.into_values().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
