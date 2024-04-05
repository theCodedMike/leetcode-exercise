//You are given an integer array nums. You want to maximize the number of
//points you get by performing the following operation any number of times:
//
//
// Pick any nums[i] and delete it to earn nums[i] points. Afterwards, you must
//delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
//
//
//
// Return the maximum number of points you can earn by applying the above
//operation some number of times.
//
//
// Example 1:
//
//
//Input: nums = [3,4,2]
//Output: 6
//Explanation: You can perform the following operations:
//- Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
//- Delete 2 to earn 2 points. nums = [].
//You earn a total of 6 points.
//
//
// Example 2:
//
//
//Input: nums = [2,2,3,3,3,4]
//Output: 9
//Explanation: You can perform the following operations:
//- Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
//
//- Delete a 3 again to earn 3 points. nums = [3].
//- Delete a 3 once more to earn 3 points. nums = [].
//You earn a total of 9 points.
//
//
// Constraints:
//
//
// 1 <= nums.length <= 2 * 10⁴
// 1 <= nums[i] <= 10⁴
//
//
// Related Topics 数组 哈希表 动态规划 👍 912 👎 0

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::max;
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        //Self::dp(nums)

        Self::sorting_then_dp(nums)
    }

    /// Time Complexity: O(N + M)
    /// Space Complexity: O(M)
    fn dp(nums: Vec<i32>) -> i32 {
        let mut max_val = 0;
        for num in &nums {
            max_val = max(max_val, *num);
        }

        let mut sum = vec![0; max_val as usize + 1];
        for num in nums {
            sum[num as usize] += num;
        }

        let rob = || {
            let (mut first, mut second) = (sum[0], max(sum[0], sum[1]));
            for i in 2..sum.len() {
                (first, second) = (second, max(first + sum[i], second));
            }
            second
        };

        rob()
    }

    /// Time Complexity: O(N*log(N))
    /// Space Complexity: O(N)
    fn sorting_then_dp(mut nums: Vec<i32>) -> i32 {
        let mut res = 0;
        nums.sort_unstable();

        let mut sum = vec![nums[0]; 1];
        let rob: fn(&mut [i32]) -> i32 = |sum| {
            let len = sum.len();
            if len == 1 {
                return sum[0];
            }

            let (mut first, mut second) = (sum[0], max(sum[0], sum[1]));
            for i in 2..len {
                (first, second) = (second, max(first + sum[i], second));
            }
            second
        };

        for i in 1..nums.len() {
            let val = nums[i];
            if val == nums[i - 1] {
                sum.last_mut().map(|last| *last += val);
            } else if val == nums[i - 1] + 1 {
                sum.push(val);
            } else {
                res += rob(&mut sum);
                sum.truncate(1);
                sum[0] = val;
            }
        }

        res += rob(&mut sum);
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
