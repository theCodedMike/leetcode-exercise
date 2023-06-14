//Given two binary strings a and b, return their sum as a binary string.
//
//
// Example 1:
// Input: a = "11", b = "1"
//Output: "100"
//
// Example 2:
// Input: a = "1010", b = "1011"
//Output: "10101"
//
//
// Constraints:
//
//
// 1 <= a.length, b.length <= 10⁴
// a and b consist only of '0' or '1' characters.
// Each string does not contain leading zeros except for the zero itself.
//
//
// Related Topics Math String Bit Manipulation Simulation 👍 8115 👎 801

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = "".to_string();
        let mut carry = 0;
        let a_len = a.len();
        let b_len = b.len();

        let (len_diff, len, short, long) = if a_len <= b_len {
            (b_len - a_len, b_len, a, b)
        } else {
            (a_len - b_len, a_len, b, a)
        };

        for i in (0..len).rev() {
            let sum = get_digit(&long, i as i32) + get_digit(&short, (i - len_diff) as i32) + carry;
            carry = sum / 2;
            result.insert_str(0, (sum % 2).to_string().as_str());
        }

        if carry == 1 {
            result.insert_str(0, "1");
        }

        result
    }
}

fn get_digit(str: &str, idx: i32) -> usize {
    if idx < 0 {
        return 0;
    }

    match str.get(idx as usize..idx as usize + 1) {
        None => 0,
        Some(x) => match x {
            "0" => 0,
            _ => 1,
        },
    }
}
//leetcode submit region end(Prohibit modification and deletion)
