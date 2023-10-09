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
        Self::two_pointers(nums)
    }
    pub fn brute_force(nums: Vec<i32>) -> Vec<i32> {
        let mut res = nums.into_iter().map(|v| v * v).collect::<Vec<_>>();
        res.sort_unstable();
        res
    }

    pub fn two_pointers(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut left = 0;
        let mut right = len - 1;
        let mut res = vec![0; len];
        let mut idx = len - 1;

        loop {
            let square_of_left = nums[left].pow(2);
            let square_of_right = nums[right].pow(2);
            if square_of_left > square_of_right {
                res[idx] = square_of_left;
                idx -= 1;
                left += 1;
            } else if square_of_left < square_of_right {
                res[idx] = square_of_right;
                idx -= 1;
                right -= 1;
            } else {
                res[idx] = square_of_right;
                if left == right {
                    break;
                }
                idx -= 1;
                res[idx] = square_of_left;
                if left + 1 == right {
                    break;
                }
                idx -= 1;
                right -= 1;
                left += 1;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
