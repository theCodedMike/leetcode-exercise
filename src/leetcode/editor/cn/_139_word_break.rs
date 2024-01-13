//Given a string s and a dictionary of strings wordDict, return true if s can
//be segmented into a space-separated sequence of one or more dictionary words.
//
// Note that the same word in the dictionary may be reused multiple times in
//the segmentation.
//
//
// Example 1:
//
//
//Input: s = "leetcode", wordDict = ["leet","code"]
//Output: true
//Explanation: Return true because "leetcode" can be segmented as "leet code".
//
//
// Example 2:
//
//
//Input: s = "applepenapple", wordDict = ["apple","pen"]
//Output: true
//Explanation: Return true because "applepenapple" can be segmented as "apple
//pen apple".
//Note that you are allowed to reuse a dictionary word.
//
//
// Example 3:
//
//
//Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 300
// 1 <= wordDict.length <= 1000
// 1 <= wordDict[i].length <= 20
// s and wordDict[i] consist of only lowercase English letters.
// All the strings of wordDict are unique.
//
//
// Related Topics Array Hash Table String Dynamic Programming Trie Memoization ?
//? 16046 👎 693

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::ops::Index;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        // todo!
        Self::recursion_segment(s.as_str(), &word_dict)
    }

    ///
    /// Time Limit Exceeded
    ///
    pub fn recursion_segment(s: &str, word_dict: &Vec<String>) -> bool {
        while !s.is_empty() {
            let matched_words = word_dict
                .iter()
                .filter(|&word| s.starts_with(word))
                .collect::<Vec<_>>();
            return if matched_words.is_empty() {
                false
            } else {
                matched_words.into_iter().any(|word| {
                    let len = word.len();
                    Self::recursion_segment(s.index(len..), word_dict)
                })
            };
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
