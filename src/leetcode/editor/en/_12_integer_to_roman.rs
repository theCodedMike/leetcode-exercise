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
// For example, 2 is written as II in Roman numeral, just two one's added
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
// Given an integer, convert it to a roman numeral.
//
//
// Example 1:
//
//
//Input: num = 3
//Output: "III"
//Explanation: 3 is represented as 3 ones.
//
//
// Example 2:
//
//
//Input: num = 58
//Output: "LVIII"
//Explanation: L = 50, V = 5, III = 3.
//
//
// Example 3:
//
//
//Input: num = 1994
//Output: "MCMXCIV"
//Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//
//
//
// Constraints:
//
//
// 1 <= num <= 3999
//
//
// Related Topics Hash Table Math String 👍 5789 👎 5139

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let num_map = HashMap::from([
            (1, "I"),
            (5, "V"),
            (10, "X"),
            (50, "L"),
            (100, "C"),
            (500, "D"),
            (1000, "M"),
            (4, "IV"),
            (9, "IX"),
            (40, "XL"),
            (90, "XC"),
            (400, "CD"),
            (900, "CM"),
        ]);

        let mut res = "".to_string();

        let mut radix = 1;
        while num != 0 {
            let rem = num % 10;

            if rem != 0 {
                let base = rem * radix;
                if num_map.contains_key(&base) {
                    res.insert_str(0, num_map[&base]);
                } else {
                    if rem < 5 {
                        res.insert_str(0, num_map[&radix].repeat(rem as usize).as_str());
                    } else {
                        res.insert_str(0, num_map[&radix].repeat((rem - 5) as usize).as_str());
                        res.insert_str(0, num_map[&(5 * radix)]);
                    }
                }
            }

            num /= 10;
            radix *= 10;
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
