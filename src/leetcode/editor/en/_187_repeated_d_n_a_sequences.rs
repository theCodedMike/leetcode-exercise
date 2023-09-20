//The DNA sequence is composed of a series of nucleotides abbreviated as 'A',
//'C', 'G', and 'T'.
//
//
// For example, "ACGAATTCCG" is a DNA sequence.
//
//
// When studying DNA, it is useful to identify repeated sequences within the
//DNA.
//
// Given a string s that represents a DNA sequence, return all the 10-letter-
//long sequences (substrings) that occur more than once in a DNA molecule. You may
//return the answer in any order.
//
//
// Example 1:
// Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
//Output: ["AAAAACCCCC","CCCCCAAAAA"]
//
// Example 2:
// Input: s = "AAAAAAAAAAAAA"
//Output: ["AAAAAAAAAA"]
//
//
// Constraints:
//
//
// 1 <= s.length <= 10⁵
// s[i] is either 'A', 'C', 'G', or 'T'.
//
//
// Related Topics Hash Table String Bit Manipulation Sliding Window Rolling
//Hash Hash Function 👍 3102 👎 501

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;
use std::ops::Index;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        //Self::double_iter(s)
        Self::single_iter(s)
    }

    ///
    /// 执行耗时:7 ms,击败了85.19% 的Rust用户
    /// 内存消耗:7.2 MB,击败了37.04% 的Rust用户
    ///
    pub fn single_iter(s: String) -> Vec<String> {
        let mut res = vec![];
        let len = s.len();
        let mut map: HashMap<&str, bool> = HashMap::new();

        for i in 10..=len {
            let substrings = s.index(i - 10..i);
            if map.contains_key(substrings) {
                if !(map[&substrings]) {
                    map.insert(substrings, true);
                    res.push(substrings.to_string());
                }
            } else {
                map.insert(substrings, false);
            }
        }

        res
    }

    ///
    /// 执行耗时:11 ms,击败了44.44% 的Rust用户
    /// 内存消耗:7.3 MB,击败了29.63% 的Rust用户
    ///
    pub fn double_iter(s: String) -> Vec<String> {
        let mut res = vec![];
        let len = s.len();
        let mut map: HashMap<&str, bool> = HashMap::new();

        for i in 0..10 {
            let mut begin = i;
            let mut end = i + 10;

            while begin < len && end <= len {
                let substrings = s.index(begin..end);
                if map.contains_key(substrings) {
                    if !(map[&substrings]) {
                        map.insert(substrings, true);
                        res.push(substrings.to_string());
                    }
                } else {
                    map.insert(substrings, false);
                }

                begin = end;
                end += 10;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
