//Given an array of integers nums sorted in non-decreasing order, find the 
//starting and ending position of a given target value. 
//
// If target is not found in the array, return [-1, -1]. 
//
// You must write an algorithm with O(log n) runtime complexity. 
//
// 
// Example 1: 
// Input: nums = [5,7,7,8,8,10], target = 8
//Output: [3,4]
// 
// Example 2: 
// Input: nums = [5,7,7,8,8,10], target = 6
//Output: [-1,-1]
// 
// Example 3: 
// Input: nums = [], target = 0
//Output: [-1,-1]
// 
// 
// Constraints: 
//
// 
// 0 <= nums.length <= 10⁵ 
// -10⁹ <= nums[i] <= 10⁹ 
// nums is a non-decreasing array. 
// -10⁹ <= target <= 10⁹ 
// 
//
// Related Topics 数组 二分查找 👍 3125 👎 0


#include <vector>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int size = static_cast<int>(nums.size());
        auto[left, right] = std::make_pair(0, size);
        vector<int> res = {-1, -1};

        while (left < right) {
            auto mid = left + (right - left) / 2;
            if (target < nums[mid])
                right = mid;
            else if (nums[mid] < target)
                left = mid + 1;
            else {
                auto[prev, next] = std::make_pair(mid, mid);
                auto[move_prev, move_next] = std::make_pair(false, false);

                while (true) {
                    move_prev = false;
                    move_next = false;
                    if (prev != 0 && nums[prev - 1] == target) {
                        move_prev = true;
                        --prev;
                    }
                    if (next != size - 1 && nums[next + 1] == target) {
                        move_next = true;
                        ++next;
                    }
                    if (!move_prev && !move_next)
                        break;
                }

                res[0] = prev;
                res[1] = next;
                break;
            }
        }

        return res;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

