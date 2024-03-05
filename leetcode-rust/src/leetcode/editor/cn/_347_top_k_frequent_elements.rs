//Given an integer array nums and an integer k, return the k most frequent
//elements. You may return the answer in any order.
//
//
// Example 1:
// Input: nums = [1,1,1,2,2,3], k = 2
//Output: [1,2]
//
// Example 2:
// Input: nums = [1], k = 1
//Output: [1]
//
//
// Constraints:
//
//
// 1 <= nums.length <= 10⁵
// -10⁴ <= nums[i] <= 10⁴
// k is in the range [1, the number of unique elements in the array].
// It is guaranteed that the answer is unique.
//
//
//
// Follow up: Your algorithm's time complexity must be better than O(n log n),
//where n is the array's size.
//
// Related Topics Array Hash Table Divide and Conquer Sorting Heap (Priority
//Queue) Bucket Sort Counting Quickselect 👍 16375 👎 595

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        //Self::use_hashmap(nums, k as usize)
        //Self::use_heap(nums, k as usize)
        Self::quick_select(nums, k as usize)
    }

    /// Time Complexity: O(n*log(n))
    ///
    /// Space Complexity: O(n)
    fn use_hashmap(nums: Vec<i32>, k: usize) -> Vec<i32> {
        let len = nums.len();
        if k == len {
            return nums;
        }

        let mut map = HashMap::with_capacity(len / 2);
        for num in nums {
            map.entry(num)
                .and_modify(|(count, _key)| {
                    *count += 1;
                })
                .or_insert((1, num));
        }

        let mut values = map.into_values().collect::<Vec<_>>();
        values.sort_unstable_by(|&v1, &v2| v2.0.cmp(&v1.0));

        values.into_iter().take(k).map(|v| v.1).collect()
    }

    /// Time Complexity: O(n*log(k))
    ///
    /// Space Complexity: O(n + k)
    fn use_heap(nums: Vec<i32>, k: usize) -> Vec<i32> {
        // O(1) time
        let len = nums.len();
        if len == k {
            return nums;
        }

        // 1. Build hash map: element and how often it appears
        // O(N) time
        let mut map = HashMap::with_capacity(len / 2);
        for num in nums {
            map.entry(num)
                .and_modify(|(count, _key)| {
                    *count += 1;
                })
                .or_insert((1, num));
        }

        // init heap 'the less frequent element first'
        let mut heap = BinaryHeap::with_capacity(k + 1);
        // 2. Keep k top frequent elements in the heap
        // O(N*log(k)) < O(N*log(N)) time
        for v in map.into_values() {
            heap.push(std::cmp::Reverse(v));
            if heap.len() > k {
                heap.pop();
            }
        }

        // 3. Build an output array
        // O(k*log(k)) time
        let mut res = vec![0; k];
        let mut i = k;
        while let Some(std::cmp::Reverse((_, num))) = heap.pop() {
            i -= 1;
            res[i] = num;
        }

        res
    }

    /// Average Time Complexity: O(n), Worst Case: O(n^2)
    ///
    /// Space Complexity: O(n)
    fn quick_select(nums: Vec<i32>, k: usize) -> Vec<i32> {
        // O(1) time
        let len = nums.len();
        if len == k {
            return nums;
        }

        // build hash map: element and how often it appears
        let mut map = HashMap::with_capacity(len / 2);
        for num in nums {
            map.entry(num).and_modify(|count| *count += 1).or_insert(1);
        }

        // array of unique elements
        let m = map.len();
        let mut values = Vec::with_capacity(m);
        for (num, count) in map {
            values.push((num, count));
        }

        // kth top frequent element is (n - k)th less frequent.
        // Do a partial sort: from less frequent to the most frequent, till
        // (n - k)th less frequent element takes its place (n - k) in a sorted array.
        // All elements on the left are less frequent.
        // All the elements on the right are more frequent.
        Self::qsort(0, m - 1, m - k, &mut values);
        // Return top k frequent elements
        values.into_iter().skip(m - k).map(|v| v.0).collect()
    }

    fn qsort(start: usize, end: usize, k: usize, values: &mut Vec<(i32, i32)>) {
        // Sort a list within start..end till kth less frequent element takes its place.
        // Base case: the list contains only one element
        if start == end {
            return;
        }
        // Select a random pivot_index
        let mut pivot_idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            % (end - start + 1) as u128
            + start as u128) as usize;
        pivot_idx = Self::partition(start, end, pivot_idx, values);

        if k < pivot_idx {
            // go left
            Self::qsort(start, pivot_idx - 1, k, values);
        } else if k > pivot_idx {
            // go right
            Self::qsort(pivot_idx + 1, end, k, values);
        } else {
            // If the pivot is in its final sorted position
            return;
        }
    }

    fn partition(
        start: usize,
        end: usize,
        pivot_idx: usize,
        values: &mut Vec<(i32, i32)>,
    ) -> usize {
        let pivot_freq = values[pivot_idx].1;
        // 1. Move pivot to end
        values.swap(pivot_idx, end);
        // Find the pivot position in a sorted list
        let mut store_idx = start;
        // 2. Move all less frequent elements to the left
        for i in start..=end {
            if values[i].1 < pivot_freq {
                values.swap(store_idx, i);
                store_idx += 1;
            }
        }
        // 3. Move the pivot to its final place
        values.swap(store_idx, end);

        store_idx
    }
}
//leetcode submit region end(Prohibit modification and deletion)
