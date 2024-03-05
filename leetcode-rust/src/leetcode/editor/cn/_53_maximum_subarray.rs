//Given an integer array nums, find the subarray with the largest sum, and
//return its sum.
//
//
// Example 1:
//
//
//Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
//Output: 6
//Explanation: The subarray [4,-1,2,1] has the largest sum 6.
//
//
// Example 2:
//
//
//Input: nums = [1]
//Output: 1
//Explanation: The subarray [1] has the largest sum 1.
//
//
// Example 3:
//
//
//Input: nums = [5,4,-1,7,8]
//Output: 23
//Explanation: The subarray [5,4,-1,7,8] has the largest sum 23.
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
//
//
//
// Follow up: If you have figured out the O(n) solution, try coding another
//solution using the divide and conquer approach, which is more subtle.
//
// Related Topics Array Divide and Conquer Dynamic Programming 👍 30807 👎 1317

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut largest_sum = i32::MIN;
        let len = nums.len();

        // todo! time limit exceeded
        for i in 0..len {
            let mut sum = 0;
            for j in i..len {
                sum += nums[j];
                if sum > largest_sum {
                    largest_sum = sum;
                }
            }
        }

        largest_sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
