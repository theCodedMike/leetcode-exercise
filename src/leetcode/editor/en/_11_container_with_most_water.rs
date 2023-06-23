//You are given an integer array height of length n. There are n vertical lines
//drawn such that the two endpoints of the iᵗʰ line are (i, 0) and (i, height[i]).
//
//
// Find two lines that together with the x-axis form a container, such that the
//container contains the most water.
//
// Return the maximum amount of water a container can store.
//
// Notice that you may not slant the container.
//
//
// Example 1:
//
//
//Input: height = [1,8,6,2,5,4,8,3,7]
//Output: 49
//Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,
//3,7]. In this case, the max area of water (blue section) the container can
//contain is 49.
//
//
// Example 2:
//
//
//Input: height = [1,1]
//Output: 1
//
//
//
// Constraints:
//
//
// n == height.length
// 2 <= n <= 10⁵
// 0 <= height[i] <= 10⁴
//
//
// Related Topics Array Two Pointers Greedy 👍 24784 👎 1328

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let len = height.len();

        let mut width = 0;
        let mut high = 0;
        let mut area = 0;

        for i in 0..(len - 1) {
            for j in (i + 1)..len {
                width = (j - i) as i32;
                high = if height[i] <= height[j] {
                    height[i]
                } else {
                    height[j]
                };

                area = width * high;
                if area > max_area {
                    max_area = area;
                }
            }
        }

        max_area
    }
}
//leetcode submit region end(Prohibit modification and deletion)
