//Given an integer array nums sorted in non-decreasing order, remove the
//duplicates in-place such that each unique element appears only once. The relative
//order of the elements should be kept the same. Then return the number of unique
//elements in nums.
//
// Consider the number of unique elements of nums to be k, to get accepted, you
//need to do the following things:
//
//
// Change the array nums such that the first k elements of nums contain the
//unique elements in the order they were present in nums initially. The remaining
//elements of nums are not important as well as the size of nums.
// Return k.
//
//
// Custom Judge:
//
// The judge will test your solution with the following code:
//
//
//int[] nums = [...]; // Input array
//int[] expectedNums = [...]; // The expected answer with correct length
//
//int k = removeDuplicates(nums); // Calls your implementation
//
//assert k == expectedNums.length;
//for (int i = 0; i < k; i++) {
//    assert nums[i] == expectedNums[i];
//}
//
//
// If all assertions pass, then your solution will be accepted.
//
//
// Example 1:
//
//
//Input: nums = [1,1,2]
//Output: 2, nums = [1,2,_]
//Explanation: Your function should return k = 2, with the first two elements
//of nums being 1 and 2 respectively.
//It does not matter what you leave beyond the returned k (hence they are
//underscores).
//
//
// Example 2:
//
//
//Input: nums = [0,0,1,1,1,2,2,3,3,4]
//Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
//Explanation: Your function should return k = 5, with the first five elements
//of nums being 0, 1, 2, 3, and 4 respectively.
//It does not matter what you leave beyond the returned k (hence they are
//underscores).
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 3 * 10⁴
// -100 <= nums[i] <= 100
// nums is sorted in non-decreasing order.
//
//
// Related Topics Array Two Pointers 👍 11215 👎 15085

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //Self::brute_force(nums)
        Self::double_pointer(nums)
    }

    pub fn brute_force(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        let mut target = nums[0];
        let mut i = 1;

        while i < len {
            if nums[i] == target {
                for j in i..len {
                    nums[j - 1] = nums[j];
                }
                len -= 1;
            } else {
                target = nums[i];
                i += 1;
            }
        }

        len as i32
    }

    pub fn double_pointer(nums: &mut Vec<i32>) -> i32 {
        let mut slow = 0;

        for fast in 1..nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
            }
        }

        slow as i32 + 1
    }
}
//leetcode submit region end(Prohibit modification and deletion)
