//Given a collection of numbers, nums, that might contain duplicates, return
//all possible unique permutations in any order.
//
//
// Example 1:
//
//
//Input: nums = [1,1,2]
//Output:
//[[1,1,2],
// [1,2,1],
// [2,1,1]]
//
//
// Example 2:
//
//
//Input: nums = [1,2,3]
//Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 8
// -10 <= nums[i] <= 10
//
//
// Related Topics Array Backtracking 👍 7750 👎 134

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        //Self::backtracking(nums)

        Self::recursion(nums)
    }

    fn backtracking(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        const DFS: fn(&mut Vec<i32>, &mut Vec<i32>, &mut Vec<Vec<i32>>) = |nums, permute, res| {
            if permute.len() == nums.len() {
                res.push(permute.clone());
                return;
            }

            for i in 0..nums.len() {
                if nums[i] == i32::MIN {
                    continue;
                }
                if i > 0 && nums[i] == nums[i - 1] {
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

    fn recursion(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        const DFS: fn(Vec<i32>) -> Vec<Vec<i32>> = |nums| {
            let mut res = vec![];
            if nums.len() <= 1 {
                res.push(nums);
                return res;
            }

            for i in 0..nums.len() {
                if i > 0 && nums[i] == nums[i - 1] {
                    continue;
                }

                let num = nums[i];
                let new_nums = nums
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &val)| if idx == i { None } else { Some(val) })
                    .collect::<Vec<_>>();
                let perms = DFS(new_nums);
                for mut perm in perms {
                    perm.push(num);
                    res.push(perm);
                }
            }

            res
        };

        DFS(nums)
    }
}
//leetcode submit region end(Prohibit modification and deletion)
