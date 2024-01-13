//Write an algorithm to determine if a number n is happy.
//
// A happy number is a number defined by the following process:
//
//
// Starting with any positive integer, replace the number by the sum of the
//squares of its digits.
// Repeat the process until the number equals 1 (where it will stay), or it
//loops endlessly in a cycle which does not include 1.
// Those numbers for which this process ends in 1 are happy.
//
//
// Return true if n is a happy number, and false if not.
//
//
// Example 1:
//
//
//Input: n = 19
//Output: true
//Explanation:
//1² + 9² = 82
//8² + 2² = 68
//6² + 8² = 100
//1² + 0² + 0² = 1
//
//
// Example 2:
//
//
//Input: n = 2
//Output: false
//
//
//
// Constraints:
//
//
// 1 <= n <= 2³¹ - 1
//
//
// Related Topics Hash Table Math Two Pointers 👍 9690 👎 1291

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        //Self::use_hash(n)
        Self::slow_fast_pointers(n)
    }

    fn use_hash(mut n: i32) -> bool {
        let mut set = HashSet::new();
        while n != 1 && !set.contains(&n) {
            set.insert(n);
            n = Self::get_next(n);
        }

        n == 1
    }

    fn get_next(mut n: i32) -> i32 {
        let mut sum = 0;
        while n != 0 {
            let i = n % 10;
            sum += i * i;
            n /= 10;
        }
        sum
    }

    fn slow_fast_pointers(n: i32) -> bool {
        let mut slow = n;
        let mut fast = n;
        loop {
            slow = Self::get_next(slow);
            fast = Self::get_next(Self::get_next(fast));
            if slow == fast || fast == 1 {
                break;
            }
        }

        fast == 1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
