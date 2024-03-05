//Given two strings s and p, return an array of all the start indices of p's
//anagrams in s. You may return the answer in any order.
//
// An Anagram is a word or phrase formed by rearranging the letters of a
//different word or phrase, typically using all the original letters exactly once.
//
//
// Example 1:
//
//
//Input: s = "cbaebabacd", p = "abc"
//Output: [0,6]
//Explanation:
//The substring with start index = 0 is "cba", which is an anagram of "abc".
//The substring with start index = 6 is "bac", which is an anagram of "abc".
//
//
// Example 2:
//
//
//Input: s = "abab", p = "ab"
//Output: [0,1,2]
//Explanation:
//The substring with start index = 0 is "ab", which is an anagram of "ab".
//The substring with start index = 1 is "ba", which is an anagram of "ab".
//The substring with start index = 2 is "ab", which is an anagram of "ab".
//
//
//
// Constraints:
//
//
// 1 <= s.length, p.length <= 3 * 10⁴
// s and p consist of lowercase English letters.
//
//
// Related Topics Hash Table String Sliding Window 👍 11892 👎 324
#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        //Self::sliding_window(s, p)
        Self::optimize_sliding_window(s, p)
    }

    fn sliding_window(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        let s_len = s.len();
        let p_len = p.len();
        if s_len < p_len {
            return res;
        }
        let mut s_counter = [0; 26];
        let mut p_counter = [0; 26];
        let p_bytes = p.as_bytes();
        let s_bytes = s.as_bytes();
        let a_u8 = b'a';

        for i in 0..p_len {
            p_counter[(p_bytes[i] - a_u8) as usize] += 1;
            s_counter[(s_bytes[i] - a_u8) as usize] += 1;
        }
        if s_counter == p_counter {
            res.push(0);
        }

        for i in p_len..s_len {
            s_counter[(s_bytes[i - p_len] - a_u8) as usize] -= 1;
            s_counter[(s_bytes[i] - a_u8) as usize] += 1;
            if s_counter == p_counter {
                res.push((i - p_len + 1) as i32);
            }
        }

        res
    }

    fn optimize_sliding_window(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        let s_len = s.len();
        let p_len = p.len();
        if s_len < p_len {
            return res;
        }

        let mut counter = [0; 26];
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let a_u8 = b'a';
        for i in 0..p_len {
            counter[(s_bytes[i] - a_u8) as usize] += 1;
            counter[(p_bytes[i] - a_u8) as usize] -= 1;
        }
        let mut diff = 0;
        for i in 0..26 {
            if counter[i] != 0 {
                diff += 1;
            }
        }
        if diff == 0 {
            res.push(0);
        }

        for i in p_len..s_len {
            let l = (s_bytes[i - p_len] - a_u8) as usize;
            if counter[l] == 1 {
                // 窗口中字母s[l]的数量与字符串p中的数量从不同变得相同
                diff -= 1;
            } else if counter[l] == 0 {
                // 窗口中字母s[l]的数量与字符串p中的数量从相同变得不同
                diff += 1;
            }
            counter[l] -= 1;

            let r = (s_bytes[i] - a_u8) as usize;
            if counter[r] == -1 {
                // 窗口中字母s[r]的数量与字符串p中的数量从不同变得相同
                diff -= 1;
            } else if counter[r] == 0 {
                // 窗口中字母s[r]的数量与字符串p中的数量从相同变得不同
                diff += 1;
            }
            counter[r] += 1;

            if diff == 0 {
                res.push((i - p_len + 1) as i32);
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
