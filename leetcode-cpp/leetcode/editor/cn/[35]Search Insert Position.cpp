//Given a sorted array of distinct integers and a target value, return the 
//index if the target is found. If not, return the index where it would be if it were 
//inserted in order. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
//
// 
//Input: nums = [1,3,5,6], target = 5
//Output: 2
// 
//
// Example 2: 
//
// 
//Input: nums = [1,3,5,6], target = 2
//Output: 1
// 
//
// Example 3: 
//
// 
//Input: nums = [1,3,5,6], target = 7
//Output: 4
// 
//
// 
// Constraints: 
//
// 
// 1 <= nums.length <= 10⁴ 
// -10⁴ <= nums[i] <= 10⁴ 
// nums contains distinct values sorted in ascending order. 
// -10⁴ <= target <= 10⁴ 
// 
//
// Related Topics 数组 二分查找 👍 2582 👎 0


#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        //return leftCloseRightOpen(nums, target);
        return leftCloseRightClose(nums, target);
    }

    int leftCloseRightOpen(const vector<int>& nums, const int target) {
        auto[left, right] = std::make_pair(0, static_cast<int>(nums.size()));

        while (left < right) {
            int mid = left + (right - left) / 2;
            if (target < nums[mid])
                right = mid;
            else if (nums[mid] < target)
                left = mid + 1;
            else
                return mid;
        }

        return left;
    }

    int leftCloseRightClose(const vector<int>& nums, const int target) {
        auto[left, right] = std::make_pair(0, static_cast<int>(nums.size()) - 1);

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (target < nums[mid])
                right = mid - 1;
            else if (nums[mid] < target)
                left = mid + 1;
            else
                return mid;
        }

        return left;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

