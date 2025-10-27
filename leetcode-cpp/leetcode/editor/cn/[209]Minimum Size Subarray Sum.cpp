//Given an array of positive integers nums and a positive integer target, 
//return the minimal length of a subarray whose sum is greater than or equal to target. 
//If there is no such subarray, return 0 instead. 
//
// 
// Example 1: 
//
// 
//Input: target = 7, nums = [2,3,1,2,4,3]
//Output: 2
//Explanation: The subarray [4,3] has the minimal length under the problem 
//constraint.
// 
//
// Example 2: 
//
// 
//Input: target = 4, nums = [1,4,4]
//Output: 1
// 
//
// Example 3: 
//
// 
//Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//Output: 0
// 
//
// 
// Constraints: 
//
// 
// 1 <= target <= 10⁹ 
// 1 <= nums.length <= 10⁵ 
// 1 <= nums[i] <= 10⁴ 
// 
//
// 
//Follow up: If you have figured out the 
//O(n) solution, try coding another solution of which the time complexity is 
//O(n log(n)).
//
// Related Topics 数组 二分查找 前缀和 滑动窗口 👍 2538 👎 0



#include <cstdint>
#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    int minSubArrayLen(int target, vector<int>& nums) {
        //return bruteForce(target, nums);
        //return binarySearch(target, nums);
        return slidingWindow(target, nums);
    }

    int bruteForce(int target, vector<int>& nums) {
        auto size = nums.size();

        for (auto width = 1; width <= size; ++width) {
            auto[beg, end] = std::make_pair(0, width);

            while (end <= size) {
                auto sum = 0;
                for (auto i = beg; i < end; ++i)
                    sum += nums[i];

                if (sum >= target)
                    return width;
                ++beg;
                end = beg + width;
            }
        }

        return 0;
    }

    int binarySearch(int target, vector<int>& nums) {
        const auto size = nums.size();
        int res = INT32_MAX;
        vector<int> sums(size + 1);

        for (auto i = 0; i < size; ++i)
            sums[i + 1] = sums[i] + nums[i];

        for (auto i = 0; i < size; ++i) {
            auto to_find = sums[i] + target;
            auto idx = std::lower_bound(sums.cbegin(), sums.cend(), to_find) - sums.begin();
            if (idx <= size)
                res = std::min(res, static_cast<int>(idx - i));
        }

        return res == INT32_MAX ? 0 : res;
    }

    int slidingWindow(int target, vector<int>& nums) {
        const auto size = nums.size();
        int res = INT32_MAX;
        int left = 0, sum = 0;

        for (auto i = 0; i < size; ++i) {
            sum += nums[i];
            while (sum >= target) {
                res = std::min(res, i - left + 1);
                sum -= nums[left];
                ++left;
            }
        }

        return res == INT32_MAX ? 0 : res;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

