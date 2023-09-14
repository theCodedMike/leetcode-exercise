//Given a 1-indexed array of integers numbers that is already sorted in non-
//decreasing order, find two numbers such that they add up to a specific target
//number. Let these two numbers be numbers[index1] and numbers[index2] where 1 <= index1
// < index2 < numbers.length.
//
// Return the indices of the two numbers, index1 and index2, added by one as an
//integer array [index1, index2] of length 2.
//
// The tests are generated such that there is exactly one solution. You may not
//use the same element twice.
//
// Your solution must use only constant extra space.
//
//
// Example 1:
//
//
//Input: numbers = [2,7,11,15], target = 9
//Output: [1,2]
//Explanation: The sum of 2 and 7 is 9. Therefore, index1 = 1, index2 = 2. We
//return [1, 2].
//
//
// Example 2:
//
//
//Input: numbers = [2,3,4], target = 6
//Output: [1,3]
//Explanation: The sum of 2 and 4 is 6. Therefore index1 = 1, index2 = 3. We
//return [1, 3].
//
//
// Example 3:
//
//
//Input: numbers = [-1,0], target = -1
//Output: [1,2]
//Explanation: The sum of -1 and 0 is -1. Therefore index1 = 1, index2 = 2. We
//return [1, 2].
//
//
//
// Constraints:
//
//
// 2 <= numbers.length <= 3 * 10⁴
// -1000 <= numbers[i] <= 1000
// numbers is sorted in non-decreasing order.
// -1000 <= target <= 1000
// The tests are generated such that there is exactly one solution.
//
//
// Related Topics Array Two Pointers Binary Search 👍 10691 👎 1291

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        Self::binary_search(numbers, target)
        //Self::double_pointer(numbers, target)
    }

    pub fn binary_search(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let len = numbers.len();
        let mut res = vec![-1, -1];
        'outer: for i in 0..len - 1 {
            let mut left = i + 1;
            let mut right = len;
            let diff = target - numbers[i];
            while left < right {
                let mid = (left + right) / 2;
                if diff < numbers[mid] {
                    right = mid;
                } else if numbers[mid] < diff {
                    left = mid + 1;
                } else {
                    res[0] = (i + 1) as i32;
                    res[1] = (mid + 1) as i32;
                    break 'outer;
                }
            }
        }

        res
    }

    pub fn double_pointer(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        let mut res = vec![-1, -1];

        while left < right {
            let sum = numbers[left] + numbers[right];
            if sum < target {
                left = left + 1;
            } else if target < sum {
                right = right - 1;
            } else {
                res[0] = (left + 1) as i32;
                res[1] = (right + 1) as i32;
                break;
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
