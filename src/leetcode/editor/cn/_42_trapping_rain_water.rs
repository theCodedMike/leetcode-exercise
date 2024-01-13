//Given n non-negative integers representing an elevation map where the width
//of each bar is 1, compute how much water it can trap after raining.
//
//
// Example 1:
//
//
//Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
//Output: 6
//Explanation: The above elevation map (black section) is represented by array [
//0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section)
//are being trapped.
//
//
// Example 2:
//
//
//Input: height = [4,2,0,3,2,5]
//Output: 9
//
//
//
// Constraints:
//
//
// n == height.length
// 1 <= n <= 2 * 10⁴
// 0 <= height[i] <= 10⁵
//
//
// Related Topics Array Two Pointers Dynamic Programming Stack Monotonic Stack ?
//? 27586 👎 381

#![allow(dead_code)]
#![allow(unused_assignments)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn trap(mut height: Vec<i32>) -> i32 {
        let mut sum = 0;
        let len = height.len();
        let mut i = 0;
        let mut j = 0;
        let mut sub_sum = 0;
        let mut out_of_bounds = false;

        loop {
            j = i + 1;
            if i >= len - 1 && j >= len {
                break;
            }
            sub_sum = 0;
            out_of_bounds = false;
            while height[i] > height[j] {
                sub_sum += height[i] - height[j];
                j += 1;
                if j >= len {
                    out_of_bounds = true;
                    break;
                }
            }

            if out_of_bounds {
                // 此时已找到最大的height，把[i, len)这部分反转
                let mut left = i;
                let mut right = len - 1;
                while left < right {
                    height.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            } else {
                sum += sub_sum;
                i = j;
            }
        }

        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
