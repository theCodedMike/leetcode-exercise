//An integer has monotone increasing digits if and only if each pair of
//adjacent digits x and y satisfy x <= y.
//
// Given an integer n, return the largest number that is less than or equal to
//n with monotone increasing digits.
//
//
// Example 1:
//
//
//Input: n = 10
//Output: 9
//
//
// Example 2:
//
//
//Input: n = 1234
//Output: 1234
//
//
// Example 3:
//
//
//Input: n = 332
//Output: 299
//
//
//
// Constraints:
//
//
// 0 <= n <= 10⁹
//
//
// Related Topics 贪心 数学 👍 451 👎 0

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        //Self::brute_force(n)

        Self::greedy(n)
    }

    fn brute_force(mut n: i32) -> i32 {
        let mut res = n;
        let is_monotone_increasing = |mut m: i32| {
            let mut rem_count = 0;
            let (mut curr, mut prev) = (0, 0);
            while m != 0 {
                prev = curr;
                curr = m % 10;
                rem_count += 1;
                if rem_count >= 2 && curr > prev {
                    return false;
                }
                m /= 10;
            }

            true
        };

        loop {
            if is_monotone_increasing(n) {
                res = n;
                break;
            }
            n -= 1;
        }

        res
    }

    fn greedy(n: i32) -> i32 {
        let mut str_n = n.to_string();
        let bytes_n = unsafe { str_n.as_bytes_mut() };
        let len = bytes_n.len();

        let mut i = 1;
        while i < len && bytes_n[i - 1] <= bytes_n[i] {
            i += 1;
        }

        if i < len {
            while i > 0 && bytes_n[i - 1] > bytes_n[i] {
                bytes_n[i - 1] -= 1;
                i -= 1;
            }
            for j in i + 1..len {
                bytes_n[j] = b'9';
            }
        }

        str_n.parse::<i32>().unwrap()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
