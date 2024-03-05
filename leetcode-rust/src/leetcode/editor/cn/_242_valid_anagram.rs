//Given two strings s and t, return true if t is an anagram of s, and false
//otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a
//different word or phrase, typically using all the original letters exactly once.
//
//
// Example 1:
// Input: s = "anagram", t = "nagaram"
//Output: true
//
// Example 2:
// Input: s = "rat", t = "car"
//Output: false
//
//
// Constraints:
//
//
// 1 <= s.length, t.length <= 5 * 10⁴
// s and t consist of lowercase English letters.
//
//
//
// Follow up: What if the inputs contain Unicode characters? How would you
//adapt your solution to such a case?
//
// Related Topics Hash Table String Sorting 👍 10856 👎 341

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        //Self::sort_then_compare(s, t)
        Self::use_hash_map(s, t)
    }

    /// Time Complexity: O(n*log(n))
    ///
    /// Space Complexity: O(log(n))
    fn sort_then_compare(mut s: String, mut t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let (s, t) = unsafe { (s.as_mut_vec(), t.as_mut_vec()) };
        s.sort();
        t.sort();
        s == t
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn use_hash_map(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = s.chars().fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
            map
        });

        for c in t.chars() {
            match map.get_mut(&c) {
                None => return false,
                Some(v) => {
                    *v -= 1;
                    if *v < 0 {
                        return false;
                    }
                }
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
