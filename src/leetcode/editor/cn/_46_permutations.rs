//Given an array nums of distinct integers, return all the possible
//permutations. You can return the answer in any order.
//
//
// Example 1:
// Input: nums = [1,2,3]
//Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
// Example 2:
// Input: nums = [0,1]
//Output: [[0,1],[1,0]]
//
// Example 3:
// Input: nums = [1]
//Output: [[1]]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 6
// -10 <= nums[i] <= 10
// All the integers of nums are unique.
//
//
// Related Topics Array Backtracking 👍 16329 👎 263

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //Self::backtracking(nums)

        Self::recursion(nums)
    }

    fn backtracking(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        const DFS: fn(&mut Vec<i32>, &mut Vec<i32>, &mut Vec<Vec<i32>>) = |nums, permute, res| {
            if permute.len() == nums.len() {
                res.push(permute.clone());
                return;
            }

            for i in 0..nums.len() {
                if nums[i] == i32::MIN {
                    continue;
                }

                permute.push(nums[i]);
                nums[i] = i32::MIN;
                DFS(nums, permute, res);
                nums[i] = permute.pop().unwrap_or_default();
            }
        };
        let mut res = vec![];

        DFS(&mut nums, &mut vec![], &mut res);

        res
    }

    fn recursion(nums: Vec<i32>) -> Vec<Vec<i32>> {
        const PERMUTE: fn(Vec<i32>) -> Vec<Vec<i32>> = |nums| {
            let mut res = vec![];
            if nums.len() <= 1 {
                res.push(nums);
                return res;
            }

            for i in 0..nums.len() {
                let num = nums[i];
                let new_nums = nums
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &val)| if idx == i { None } else { Some(val) })
                    .collect::<Vec<_>>();
                let perms = PERMUTE(new_nums);
                for mut perm in perms {
                    perm.push(num);
                    res.push(perm);
                }
            }

            res
        };

        PERMUTE(nums)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
