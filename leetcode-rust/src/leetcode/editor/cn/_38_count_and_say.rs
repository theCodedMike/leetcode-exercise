//The count-and-say sequence is a sequence of digit strings defined by the
//recursive formula:
//
//
// countAndSay(1) = "1"
// countAndSay(n) is the way you would "say" the digit string from countAndSay(
//n-1), which is then converted into a different digit string.
//
//
// To determine how you "say" a digit string, split it into the minimal number
//of substrings such that each substring contains exactly one unique digit. Then
//for each substring, say the number of digits, then say the digit. Finally,
//concatenate every said digit.
//
// For example, the saying and conversion for digit string "3322251":
//
// Given a positive integer n, return the nᵗʰ term of the count-and-say
//sequence.
//
//
// Example 1:
//
//
//Input: n = 1
//Output: "1"
//Explanation: This is the base case.
//
//
// Example 2:
//
//
//Input: n = 4
//Output: "1211"
//Explanation:
//countAndSay(1) = "1"
//countAndSay(2) = say "1"    = one 1                   = "11"
//countAndSay(3) = say "11"   = two 1's                 = "21"
//countAndSay(4) = say "21"   = one 2 + one 1           = "12" + "11"        = "1211"
//countAndSay(5) = say "1211" = one 1 + one 2 + two 1's = "11" + "12" + "21" = "111221"
//
//
// Constraints:
//
//
// 1 <= n <= 30
//
//
// Related Topics String 👍 3298 👎 6933

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = "1".to_string();

        for _ in 2..=n {
            res = Solution::gen_new_digit_str(&res);
        }

        res
    }

    fn gen_new_digit_str(old: &str) -> String {
        let mut digit_str = "".to_string();
        let mut prev = '0';
        let mut count = 0;
        for (idx, c) in old.chars().enumerate() {
            if idx == 0 {
                prev = c; // initialize prev
                count = 1;
            } else {
                if c == prev {
                    count += 1;
                } else {
                    Solution::push_str(&mut digit_str, count, prev);
                    prev = c;
                    count = 1;
                }
            }
        }
        Solution::push_str(&mut digit_str, count, prev);

        digit_str
    }

    fn push_str(digit_str: &mut String, count: i32, prev: char) {
        digit_str.push_str(count.to_string().as_str());
        digit_str.push(prev);
    }
}
//leetcode submit region end(Prohibit modification and deletion)
