//You are given a 0-indexed array of integers nums of length n. You are
//initially positioned at nums[0].
//
// Each element nums[i] represents the maximum length of a forward jump from
//index i. In other words, if you are at nums[i], you can jump to any nums[i + j]
//where:
//
//
// 0 <= j <= nums[i] and
// i + j < n
//
//
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are
//generated such that you can reach nums[n - 1].
//
//
// Example 1:
//
//
//Input: nums = [2,3,1,1,4]
//Output: 2
//Explanation: The minimum number of jumps to reach the last index is 2. Jump 1
//step from index 0 to 1, then 3 steps to the last index.
//
//
// Example 2:
//
//
//Input: nums = [2,3,0,1,4]
//Output: 2
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// 0 <= nums[i] <= 1000
// It's guaranteed that you can reach nums[n - 1].
//
//
// Related Topics Array Dynamic Programming Greedy 👍 12676 👎 444

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        //Self::reverse_traversal(nums)

        Self::greedy(nums)
    }

    fn reverse_traversal(nums: Vec<i32>) -> i32 {
        let mut pos = nums.len() - 1;
        let mut steps = 0;

        while pos != 0 {
            for i in 0..pos {
                if i + nums[i] as usize >= pos {
                    pos = i;
                    steps += 1;
                    break;
                }
            }
        }

        steps
    }

    fn greedy(nums: Vec<i32>) -> i32 {
        let (mut max_pos, len, mut end) = (0, nums.len(), 0);
        let mut steps = 0;

        for i in 0..len - 1 {
            if max_pos >= i {
                max_pos = std::cmp::max(max_pos, i + nums[i] as usize);
                if i == end {
                    end = max_pos;
                    steps += 1;
                }
            }
        }

        steps
    }
}
//leetcode submit region end(Prohibit modification and deletion)
