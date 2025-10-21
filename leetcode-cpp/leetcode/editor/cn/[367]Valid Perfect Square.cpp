//Given a positive integer num, return true if num is a perfect square or false 
//otherwise. 
//
// A perfect square is an integer that is the square of an integer. In other 
//words, it is the product of some integer with itself. 
//
// You must not use any built-in library function, such as sqrt. 
//
// 
// Example 1: 
//
// 
//Input: num = 16
//Output: true
//Explanation: We return true because 4 * 4 = 16 and 4 is an integer.
// 
//
// Example 2: 
//
// 
//Input: num = 14
//Output: false
//Explanation: We return false because 3.742 * 3.742 = 14 and 3.742 is not an 
//integer.
// 
//
// 
// Constraints: 
//
// 
// 1 <= num <= 2³¹ - 1 
// 
//
// Related Topics 数学 二分查找 👍 614 👎 0


#include <iostream>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    bool isPerfectSquare(int num) {
        //return leftCloseRightOpen(num);
        return leftCloseRightClose(num);
    }

    bool leftCloseRightOpen(int num) {
        const long number = num;
        auto[left, right] = std::make_pair(0, number + 1);

        while (left < right) {
            const auto mid = left + (right - left) / 2;
            const auto square = mid * mid;
            if (square > number)
                right = mid;
            else if (square < number)
                left = mid + 1;
            else
                return true;
        }

        return false;
    }

    bool leftCloseRightClose(int x) {
        const long number = x;
        auto[left, right] = std::make_pair(0, number);

        while (left <= right) {
            const auto mid = left + (right - left) / 2;
            const auto square = mid * mid;
            if (square > number)
                right = mid - 1;
            else if (square < number)
                left = mid + 1;
            else
                return true;
        }

        return false;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

