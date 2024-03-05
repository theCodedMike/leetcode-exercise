//Roman numerals are represented by seven different symbols: I, V, X, L, C, D
//and M.
//
//
//Symbol       Value
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
//
// For example, 2 is written as II in Roman numeral, just two ones added
//together. 12 is written as XII, which is simply X + II. The number 27 is written as
//XXVII, which is XX + V + II.
//
// Roman numerals are usually written largest to smallest from left to right.
//However, the numeral for four is not IIII. Instead, the number four is written as
//IV. Because the one is before the five we subtract it making four. The same
//principle applies to the number nine, which is written as IX. There are six
//instances where subtraction is used:
//
//
// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
//
//
// Given a roman numeral, convert it to an integer.
//
//
// Example 1:
//
//
//Input: s = "III"
//Output: 3
//Explanation: III = 3.
//
//
// Example 2:
//
//
//Input: s = "LVIII"
//Output: 58
//Explanation: L = 50, V= 5, III = 3.
//
//
// Example 3:
//
//
//Input: s = "MCMXCIV"
//Output: 1994
//Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 15
// s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
// It is guaranteed that s is a valid roman numeral in the range [1, 3999].
//
//
// Related Topics Hash Table Math String 👍 10721 👎 621

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut map = HashMap::new();
        map.insert("I", 1);
        map.insert("V", 5);
        map.insert("X", 10);
        map.insert("L", 50);
        map.insert("C", 100);
        map.insert("D", 500);
        map.insert("M", 1000);

        let mut sum = 0;
        let len = s.len();
        let mut i = 0;
        while i < len {
            let cur_char = s.get(i..i + 1).unwrap();
            let cur_val = map[cur_char];
            if i + 2 <= len {
                let next_char = s.get(i + 1..i + 2).unwrap();
                let next_val = map[next_char];
                if cur_val >= next_val {
                    sum += cur_val;
                    i += 1;
                } else {
                    sum += next_val - cur_val;
                    i += 2;
                }
            } else {
                sum += cur_val;
                i += 1;
            }
        }

        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
