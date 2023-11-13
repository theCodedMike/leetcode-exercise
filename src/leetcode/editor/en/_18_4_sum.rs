//Given an array nums of n integers, return an array of all the unique
//quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:
//
//
// 0 <= a, b, c, d < n
// a, b, c, and d are distinct.
// nums[a] + nums[b] + nums[c] + nums[d] == target
//
//
// You may return the answer in any order.
//
//
// Example 1:
//
//
//Input: nums = [1,0,-1,0,-2,2], target = 0
//Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
//
//
// Example 2:
//
//
//Input: nums = [2,2,2,2,2], target = 8
//Output: [[2,2,2,2]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 200
// -10⁹ <= nums[i] <= 10⁹
// -10⁹ <= target <= 10⁹
//
//
// Related Topics Array Two Pointers Sorting 👍 9608 👎 1141

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        //Self::sorting_then_2_pointers(nums, target as i64)
        Self::sorting_then_4_pointers(nums, target as i64)
    }

    /// Time Complexity: O(n^3)
    ///
    /// Space Complexity: O(n)
    fn sorting_then_2_pointers(mut nums: Vec<i32>, target: i64) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut res = vec![];
        if len < 4 {
            return res;
        }
        nums.sort_unstable();

        for i in 0..len - 3 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            for j in i + 1..len - 2 {
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let mut m = j + 1;
                let mut n = len - 1;
                while m < n {
                    let sum = nums[i] as i64 + nums[j] as i64 + nums[m] as i64 + nums[n] as i64;
                    if sum > target {
                        n -= 1;
                    } else if sum < target {
                        m += 1;
                    } else {
                        res.push(vec![nums[i], nums[j], nums[m], nums[n]]);
                        loop {
                            m += 1;
                            if nums[m] != nums[m - 1] || m >= n {
                                break;
                            }
                        }
                        loop {
                            n -= 1;
                            if nums[n] != nums[n + 1] || m >= n {
                                break;
                            }
                        }
                    }
                }
            }
        }

        res
    }

    /// Time Complexity: O(n^2)
    ///
    /// Space Complexity: O(n)
    fn sorting_then_4_pointers(mut nums: Vec<i32>, target: i64) -> Vec<Vec<i32>> {
        let len = nums.len();
        if len < 4 {
            return vec![];
        }
        nums.sort_unstable();
        let mut res = vec![];

        let mut i = 0;
        let mut j = len - 1;
        while i < j {
            let mut m = i + 1;
            let mut n = j - 1;
            while m < n {
                let sum = nums[i] as i64 + nums[j] as i64 + nums[m] as i64 + nums[n] as i64;
                if sum > target {
                    n -= 1;
                } else if sum < target {
                    m += 1;
                } else {
                    res.push(vec![nums[i], nums[m], nums[n], nums[j]]);
                    loop {
                        m += 1;
                        if nums[m] != nums[m - 1] || m >= n {
                            break;
                        }
                    }
                    loop {
                        n -= 1;
                        if nums[n] != nums[n + 1] || m >= n {
                            break;
                        }
                    }
                }
            }

            if i + 3 <= j {
                loop {
                    j -= 1;
                    if nums[j] != nums[j + 1] || i + 3 > j {
                        break;
                    }
                }
            } else {
                j = len - 1;
                loop {
                    i += 1;
                    if nums[i] != nums[i - 1] || i + 3 > j {
                        break;
                    }
                }
            }
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
