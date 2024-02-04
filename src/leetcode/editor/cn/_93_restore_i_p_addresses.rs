//A valid IP address consists of exactly four integers separated by single dots.
// Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.
//
//
// For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011
//.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
//
//
// Given a string s containing only digits, return all possible valid IP
//addresses that can be formed by inserting dots into s. You are not allowed to reorder
//or remove any digits in s. You may return the valid IP addresses in any order.
//
//
// Example 1:
//
//
//Input: s = "25525511135"
//Output: ["255.255.11.135","255.255.111.35"]
//
//
// Example 2:
//
//
//Input: s = "0000"
//Output: ["0.0.0.0"]
//
//
// Example 3:
//
//
//Input: s = "101023"
//Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
//
//
//
// Constraints:
//
//
// 1 <= s.length <= 20
// s consists of digits only.
//
//
// Related Topics String Backtracking 👍 4846 👎 759

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        Self::backtracking(s)
    }

    fn backtracking(s: String) -> Vec<String> {
        const DFS: for<'a> fn(usize, &'a str, &mut Vec<&'a str>, &mut Vec<String>) =
            |i, s, address, res| {
                if address.len() == 4 {
                    res.push(address.join("."));
                    return;
                }

                let start = if address.len() != 3 { i + 1 } else { s.len() };
                for j in start..=s.len() {
                    let substr = &s[i..j];
                    if substr.is_empty() {
                        break;
                    }
                    if substr.starts_with('0') && substr.len() > 1 {
                        break;
                    }
                    if substr.parse::<usize>().is_ok_and(|num| num > 255) {
                        break;
                    }

                    address.push(substr);
                    DFS(j, s, address, res);
                    address.pop();
                }
            };
        let mut res = vec![];

        DFS(0, &s, &mut vec![], &mut res);

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
