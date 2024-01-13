//Given a string containing digits from 2-9 inclusive, return all possible
//letter combinations that the number could represent. Return the answer in any order.
//
//
// A mapping of digits to letters (just like on the telephone buttons) is given
//below. Note that 1 does not map to any letters.
//
//
// Example 1:
//
//
//Input: digits = "23"
//Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
//
//
// Example 2:
//
//
//Input: digits = ""
//Output: []
//
//
// Example 3:
//
//
//Input: digits = "2"
//Output: ["a","b","c"]
//
//
//
// Constraints:
//
//
// 0 <= digits.length <= 4
// digits[i] is a digit in the range ['2', '9'].
//
//
// Related Topics Hash Table String Backtracking 👍 15418 👎 852

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut res = vec![];
        if digits.is_empty() {
            return res;
        }

        let mut map = HashMap::new();
        map.insert('2', "abc");
        map.insert('3', "def");
        map.insert('4', "ghi");
        map.insert('5', "jkl");
        map.insert('6', "mno");
        map.insert('7', "pqrs");
        map.insert('8', "tuv");
        map.insert('9', "wxyz");

        for c in digits.chars() {
            res = two_words_multiply(&res, map[&c]);
        }

        res
    }
}

fn two_words_multiply(res: &Vec<String>, word: &str) -> Vec<String> {
    if res.is_empty() {
        word.chars().map(|c| c.to_string()).collect()
    } else {
        let mut new_res = Vec::with_capacity(res.len() * word.len());

        for ori in res {
            for c in word.chars() {
                let mut s = ori.to_string();
                s.push(c);
                new_res.push(s);
            }
        }

        new_res
    }
}

//leetcode submit region end(Prohibit modification and deletion)
