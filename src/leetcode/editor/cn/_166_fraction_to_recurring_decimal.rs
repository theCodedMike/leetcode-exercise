//Given two integers representing the numerator and denominator of a fraction,
//return the fraction in string format.
//
// If the fractional part is repeating, enclose the repeating part in
//parentheses.
//
// If multiple answers are possible, return any of them.
//
// It is guaranteed that the length of the answer string is less than 10⁴ for
//all the given inputs.
//
//
// Example 1:
//
//
//Input: numerator = 1, denominator = 2
//Output: "0.5"
//
//
// Example 2:
//
//
//Input: numerator = 2, denominator = 1
//Output: "2"
//
//
// Example 3:
//
//
//Input: numerator = 4, denominator = 333
//Output: "0.(012)"
//
//
//
// Constraints:
//
//
// -2³¹ <= numerator, denominator <= 2³¹ - 1
// denominator != 0
//
//
// Related Topics Hash Table Math String 👍 1973 👎 3548

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut num = numerator as i64;
        let mut den = denominator as i64;

        // 如果能整除
        if num % den == 0 {
            return (num / den).to_string();
        }

        // 如果除数与被除数符号不同
        let mut res = String::new();
        if num * den < 0 {
            res.push('-');
        }

        // 整数部分
        num = num.abs();
        den = den.abs();
        let int_part = num / den;
        res.push_str(&int_part.to_string());
        res.push('.');

        // 小数部分
        let mut map = HashMap::new();
        let mut rem = num % den;
        let mut idx = 0;
        let mut frac_part = "".to_string();
        while rem != 0 && !map.contains_key(&rem) {
            map.insert(rem, idx);
            rem *= 10;
            frac_part.push_str(&(rem / den).to_string());
            rem %= den;
            idx += 1;
        }
        if rem != 0 {
            let idx = map[&rem];
            frac_part.insert(idx, '(');
            frac_part.push(')');
        }
        res.push_str(&frac_part);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
