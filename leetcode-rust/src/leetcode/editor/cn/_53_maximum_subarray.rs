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
use std::cmp::{max, min};
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::dp(nums)

        //Self::sum_of_prefix(nums)

        //Self::divide_and_conquer(nums)
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn dp(nums: Vec<i32>) -> i32 {
        let (mut pre, mut res) = (0, nums[0]);

        for num in nums {
            pre = max(pre + num, num);
            res = max(res, pre);
        }

        res
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(1)
    fn sum_of_prefix(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let (mut min_pre_sum, mut pre_sum) = (0, 0);

        for num in nums {
            pre_sum += num;
            res = max(res, pre_sum - min_pre_sum);
            min_pre_sum = min(min_pre_sum, pre_sum);
        }

        res
    }

    /// Time Complexity: O(n)
    /// Space Complexity: O(log(n))
    fn divide_and_conquer(nums: Vec<i32>) -> i32 {
        struct Status {
            l_sum: i32,
            r_sum: i32,
            m_sum: i32,
            i_sum: i32,
        }

        const PUSH_UP: fn(Status, Status) -> Status = |l, r| Status {
            l_sum: max(l.l_sum, l.i_sum + r.l_sum),
            r_sum: max(r.r_sum, r.i_sum + l.r_sum),
            m_sum: max(max(l.m_sum, r.m_sum), l.r_sum + r.l_sum),
            i_sum: l.i_sum + r.i_sum,
        };

        const GET: fn(&[i32], usize, usize) -> Status = |nums, l, r| {
            if l == r {
                return Status {
                    l_sum: nums[l],
                    r_sum: nums[l],
                    m_sum: nums[l],
                    i_sum: nums[l],
                };
            }

            let m = (l + r) >> 1;
            let l_sub = GET(nums, l, m);
            let r_sub = GET(nums, m + 1, r);

            PUSH_UP(l_sub, r_sub)
        };

        GET(&nums, 0, nums.len() - 1).m_sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)
