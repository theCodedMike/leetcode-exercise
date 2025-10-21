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


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public bool IsPerfectSquare(int num)
    {
        //return LeftCloseRightOpen(num);
        return LeftCloseRightClose(num);
    }

    bool LeftCloseRightOpen(int num)
    {
        long number = num;
        (long left, long right) = (0, number + 1);

        while (left < right)
        {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (square > number)
                right = mid;
            else if (square < number)
                left = mid + 1;
            else
                return true;
        }
        
        return false;
    }
    
    bool LeftCloseRightClose(int num)
    {
        long number = num;
        (long left, long right) = (0, number);

        while (left <= right)
        {
            long mid = left + (right - left) / 2;
            long square = mid * mid;
            if (square > number)
                right = mid - 1;
            else if (square < number)
                left = mid + 1;
            else
                return true;
        }
        
        return false;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
