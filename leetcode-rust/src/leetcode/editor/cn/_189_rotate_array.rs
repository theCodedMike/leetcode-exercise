//Given an integer array nums, rotate the array to the right by k steps, where
//k is non-negative.
//
//
// Example 1:
//
//
//Input: nums = [1,2,3,4,5,6,7], k = 3
//Output: [5,6,7,1,2,3,4]
//Explanation:
//rotate 1 steps to the right: [7,1,2,3,4,5,6]
//rotate 2 steps to the right: [6,7,1,2,3,4,5]
//rotate 3 steps to the right: [5,6,7,1,2,3,4]
//
//
// Example 2:
//
//
//Input: nums = [-1,-100,3,99], k = 2
//Output: [3,99,-1,-100]
//Explanation:
//rotate 1 steps to the right: [99,-1,-100,3]
//rotate 2 steps to the right: [3,99,-1,-100]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -2³¹ <= nums[i] <= 2³¹ - 1
// 0 <= k <= 10⁵
//
//
//
// Follow up:
//
//
// Try to come up with as many solutions as you can. There are at least three
//different ways to solve this problem.
// Could you do it in-place with O(1) extra space?
//
//
// Related Topics Array Math Two Pointers 👍 16223 👎 1772

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
enum RotateDirection {
    Left,
    Right,
}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        //Self::rotate_right(nums, k)
        Self::rotate_left_or_right(nums, k)
    }

    ///
    /// 一直向右旋转
    ///
    /// 执行耗时:3 ms,击败了99.80% 的Rust用户
    /// 内存消耗:3.8 MB,击败了14.02% 的Rust用户
    pub fn rotate_right(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        if k == 0 {
            return;
        }

        let mut tmp = Vec::with_capacity(k);
        for i in 0..k {
            tmp.push(nums[i]);
        }
        for i in (k..len).rev() {
            let new_i = (i + k) % len;
            nums[new_i] = nums[i];
        }
        for i in 0..k {
            nums[(i + k) % len] = tmp[i]
        }
    }

    /// 如果 k > len/2, 则向左旋转；如果 k <= len / 2, 则向右旋转
    ///
    /// 执行耗时:9 ms,击败了70.33% 的Rust用户
    /// 内存消耗:3.7 MB,击败了47.97% 的Rust用户
    pub fn rotate_left_or_right(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        if k == 0 {
            return;
        }
        let (cap, dir) = if k <= len / 2 {
            // Rotate Right
            (k, RotateDirection::Right)
        } else {
            // Rotate Left
            (len - k, RotateDirection::Left)
        };

        let mut tmp = Vec::with_capacity(cap);
        for i in 0..cap {
            tmp.push(nums[i]);
        }

        match dir {
            RotateDirection::Left => {
                let calc_f = |idx: usize, cap: usize| idx - cap;
                for i in cap..len {
                    nums[calc_f(i, cap)] = nums[i];
                }
            }
            RotateDirection::Right => {
                let calc_f = |idx: usize, cap: usize| (idx + cap) % len;
                for i in (cap..len).rev() {
                    nums[calc_f(i, cap)] = nums[i];
                }
            }
        }

        for i in 0..cap {
            nums[(i + k) % len] = tmp[i];
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)
