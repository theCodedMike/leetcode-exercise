//Given an array of integers nums which is sorted in ascending order, and an 
//integer target, write a function to search target in nums. If target exists, then 
//return its index. Otherwise, return -1. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
//
// 
//Input: nums = [-1,0,3,5,9,12], target = 9
//Output: 4
//Explanation: 9 exists in nums and its index is 4
// 
//
// Example 2: 
//
// 
//Input: nums = [-1,0,3,5,9,12], target = 2
//Output: -1
//Explanation: 2 does not exist in nums so return -1
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁴ 
// -10⁴ < nums[i], target < 10⁴ 
// All the integers in nums are unique. 
// nums is sorted in ascending order. 
// 
//
// Related Topics 数组 二分查找 👍 1801 👎 0


#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    int search(vector<int>& nums, int target) {
        //return full_match(nums, target);
        //return match_right(nums, target);
        //return match_left(nums, target);
        return use_std(nums, target);
    }

    int full_match(vector<int>& nums, int target) {
        auto left = 0;
        auto right = nums.size();

        while (left < right) {
            auto mid = left + ((right - left) >> 1);
            if (target < nums[mid])
                right = mid;
            else if (target > nums[mid])
                left = mid + 1;
            else
                return mid;
        }

        return -1;
    }

    int match_right(vector<int>& nums, int target) {
        auto left = 0;
        auto right = nums.size();

        while (left < right) {
            auto mid = left + ((right - left) >> 1);
            if (target >= nums[mid])
                left = mid + 1;
            else
                right = mid;
        }

        if (left > 0 && nums[left - 1] == target)
            return left - 1;
        return -1;
    }

    int match_left(vector<int>& nums, int target) {
        auto left = 0;
        auto right = nums.size();

        while (left < right) {
            auto mid = left + ((right - left) >> 1);
            if (target <= nums[mid])
                right = mid;
            else
                left = mid + 1;
        }

        if (left < nums.size() && nums[left] == target)
            return left;
        return -1;
    }

    int use_std(vector<int>& nums, int target) {
        if (std::binary_search(nums.cbegin(), nums.cend(), target))
            return std::lower_bound(nums.cbegin(), nums.cend(), target) - nums.cbegin();

        return -1;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

