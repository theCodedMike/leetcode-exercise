//Given a non-negative integer x, return the square root of x rounded down to
//the nearest integer. The returned integer should be non-negative as well.
//
// You must not use any built-in exponent function or operator.
//
//
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
//
//
//
// Example 1:
//
//
//Input: x = 4
//Output: 2
//Explanation: The square root of 4 is 2, so we return 2.
//
//
// Example 2:
//
//
//Input: x = 8
//Output: 2
//Explanation: The square root of 8 is 2.82842..., and since we round it down
//to the nearest integer, 2 is returned.
//
//
//
// Constraints:
//
//
// 0 <= x <= 2³¹ - 1
//
//
// Related Topics Math Binary Search 👍 6564 👎 4095

#![allow(dead_code)]
#![allow(unused_assignments)]

/// 解决方案1: 从n=1开始迭代，判断n^2<=x，n每次递增1
/// 解决方案2: 使用二分搜索，low = 1，high = x / 2
///
pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        //Self::left_close_right_open(x)
        Self::left_close_right_close(x)
    }

    fn left_close_right_open(x: i32) -> i32 {
        let x = x as i64;
        let (mut left, mut right) = (0_i64, x + 1);

        while left < right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;
            if square > x {
                right = mid;
            } else if square < x {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        left as i32 - 1
    }

    fn left_close_right_close(x: i32) -> i32 {
        let x = x as i64;
        let (mut left, mut right) = (0_i64, x);

        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid * mid;
            if square > x {
                right = mid - 1;
            } else if square < x {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        left as i32 - 1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
