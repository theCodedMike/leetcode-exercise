//Given an integer array nums sorted in non-decreasing order, return an array
//of the squares of each number sorted in non-decreasing order.
//
//
// Example 1:
//
//
//Input: nums = [-4,-1,0,3,10]
//Output: [0,1,9,16,100]
//Explanation: After squaring, the array becomes [16,1,0,9,100].
//After sorting, it becomes [0,1,9,16,100].
//
//
// Example 2:
//
//
//Input: nums = [-7,-3,2,3,11]
//Output: [4,9,9,49,121]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁴
// -10⁴ <= nums[i] <= 10⁴
// nums is sorted in non-decreasing order.
//
//
//
//Follow up: Squaring each element and sorting the new array is very trivial,
//could you find an
//O(n) solution using a different approach?
//
// Related Topics Array Two Pointers Sorting 👍 8385 👎 205

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        //Self::brute_force(nums)
        //Self::two_pointers_1(nums)
        Self::two_pointers_2(nums)
    }
    pub fn brute_force(mut nums: Vec<i32>) -> Vec<i32> {
        for num in &mut nums {
            *num = *num * *num;
        }
        nums.sort_unstable();

        nums
    }

    pub fn two_pointers_1(nums: Vec<i32>) -> Vec<i32> {
        let (mut res, mut idx) = (vec![0; nums.len()], nums.len() as i32 - 1);
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);

        while left <= right {
            let left_square = nums[left as usize] * nums[left as usize];
            let right_square = nums[right as usize] * nums[right as usize];
            if left_square > right_square {
                res[idx as usize] = left_square;
                left += 1;
                idx -= 1;
            } else if left_square < right_square {
                res[idx as usize] = right_square;
                right -= 1;
                idx -= 1;
            } else {
                res[idx as usize] = right_square;
                if left != right {
                    res[idx as usize - 1] = left_square;
                }
                left += 1;
                right -= 1;
                idx -= 2;
            }
        }

        res
    }

    pub fn two_pointers_2(nums: Vec<i32>) -> Vec<i32> {
        let (mut res, mut idx) = (vec![0; nums.len()], nums.len() as i32 - 1);
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);

        while left <= right {
            let left_square = nums[left as usize] * nums[left as usize];
            let right_square = nums[right as usize] * nums[right as usize];
            if left_square > right_square {
                res[idx as usize] = left_square;
                left += 1;
                idx -= 1;
            } else {
                res[idx as usize] = right_square;
                right -= 1;
                idx -= 1;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
