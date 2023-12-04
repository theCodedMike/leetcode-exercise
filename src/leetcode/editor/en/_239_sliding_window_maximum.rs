//You are given an array of integers nums, there is a sliding window of size k
//which is moving from the very left of the array to the very right. You can only
//see the k numbers in the window. Each time the sliding window moves right by one
//position.
//
// Return the max sliding window.
//
//
// Example 1:
//
//
//Input: nums = [1,3,-1,-3,5,3,6,7], k = 3
//Output: [3,3,5,5,6,7]
//Explanation:
//Window position                Max
//---------------               -----
//[1  3  -1] -3  5  3  6  7       3
// 1 [3  -1  -3] 5  3  6  7       3
// 1  3 [-1  -3  5] 3  6  7       5
// 1  3  -1 [-3  5  3] 6  7       5
// 1  3  -1  -3 [5  3  6] 7       6
// 1  3  -1  -3  5 [3  6  7]      7
//
//
// Example 2:
//
//
//Input: nums = [1], k = 1
//Output: [1]
//
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
// 1 <= k <= nums.length
//
//
// Related Topics Array Queue Sliding Window Heap (Priority Queue) Monotonic
//Queue 👍 17410 👎 609

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{BinaryHeap, VecDeque};
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        //Self::priority_queue(nums, k as usize)
        Self::monotonic_queue(nums, k as usize)
        //Self::split_block(nums, k as usize)
    }

    /// Time Complexity: O(n*log(n))
    ///
    /// Space Complexity: O(n)
    fn priority_queue(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let len = nums.len();
        let mut res = Vec::with_capacity(len - k + 1);
        let mut heap = BinaryHeap::with_capacity(len);

        for i in 0..k {
            heap.push((nums[i], i));
        }
        res.push((*heap.peek().unwrap()).0);

        for i in k..len {
            heap.push((nums[i], i));
            while let Some((max, idx)) = heap.peek() {
                if *idx <= i - k {
                    heap.pop();
                } else {
                    res.push(*max);
                    break;
                }
            }
        }

        res
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(k)
    fn monotonic_queue(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let len = nums.len();
        let mut res = Vec::with_capacity(len - k + 1);
        let mut deque = VecDeque::with_capacity(k);
        let push_back = |deque: &mut VecDeque<usize>, i: usize| {
            while let Some(&back) = deque.back() {
                if nums[i] >= nums[back] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
        };

        for i in 0..k {
            push_back(&mut deque, i);
        }
        res.push(nums[*deque.front().unwrap()]);

        for i in k..len {
            push_back(&mut deque, i);

            while let Some(&front) = deque.front() {
                if front <= i - k {
                    deque.pop_front();
                } else {
                    res.push(nums[front]);
                    break;
                }
            }
        }

        res
    }

    /// Time Complexity: O(n)
    ///
    /// Space Complexity: O(n)
    fn split_block(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let len = nums.len();
        let mut prefix_max = vec![0; len];
        let mut suffix_max = vec![0; len];

        for mut i in 0..len {
            if i % k == 0 {
                prefix_max[i] = nums[i];
            } else {
                prefix_max[i] = std::cmp::max(prefix_max[i - 1], nums[i]);
            }
            i = len - 1 - i;
            if i == len - 1 || (i + 1) % k == 0 {
                suffix_max[i] = nums[i];
            } else {
                suffix_max[i] = std::cmp::max(suffix_max[i + 1], nums[i]);
            }
        }

        let mut res = Vec::with_capacity(len - k + 1);
        for i in 0..=len - k {
            res.push(std::cmp::max(suffix_max[i], prefix_max[i + k - 1]));
        }

        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
