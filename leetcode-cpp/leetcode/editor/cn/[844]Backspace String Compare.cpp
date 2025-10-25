//Given two strings s and t, return true if they are equal when both are typed 
//into empty text editors. '#' means a backspace character. 
//
// Note that after backspacing an empty text, the text will continue empty. 
//
// 
// Example 1: 
//
// 
//Input: s = "ab#c", t = "ad#c"
//Output: true
//Explanation: Both s and t become "ac".
// 
//
// Example 2: 
//
// 
//Input: s = "ab##", t = "c#d#"
//Output: true
//Explanation: Both s and t become "".
// 
//
// Example 3: 
//
// 
//Input: s = "a#c", t = "b"
//Output: false
//Explanation: s becomes "c" while t becomes "b".
// 
//
// 
// Constraints: 
//
// 
// 1 <= s.length, t.length <= 200 
// s and t only contain lowercase letters and '#' characters. 
// 
//
// 
// Follow up: Can you solve it in O(n) time and O(1) space? 
//
// Related Topics 栈 双指针 字符串 模拟 👍 841 👎 0


#include <iostream>
using namespace std;

//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    bool backspaceCompare(string s, string t) {
        //return buildString(s, t);
        return twoPointers(s, t);
    }

    bool buildString(string s, string t) {
        auto build = [](const string &str) -> string {
            string res;

            for (const char ch: str) {
                if (ch == '#') {
                    if (!res.empty())
                        res.pop_back();
                } else
                    res.push_back(ch);
            }

            return res;
        };

        return build(s) == build(t);
    }

    bool twoPointers(string s, string t) {
        auto[sIdx, tIdx] = std::make_pair(
            static_cast<int>(s.size()) - 1, static_cast<int>(t.size()) - 1);
        auto[sSharpCount, tSharpCount] = std::make_pair(0, 0);

        while (sIdx >= 0 || tIdx >= 0) {
            while (sIdx >= 0) {
                if (s[sIdx] == '#') {
                    sSharpCount++;
                    sIdx--;
                } else if (sSharpCount > 0) {
                    sSharpCount--;
                    sIdx--;
                } else
                    break;
            }
            while (tIdx >= 0) {
                if (t[tIdx] == '#') {
                    tSharpCount++;
                    tIdx--;
                } else if (tSharpCount > 0) {
                    tSharpCount--;
                    tIdx--;
                } else
                    break;
            }

            if (sIdx >= 0 && tIdx >= 0 && s[sIdx] != t[tIdx])
                return false;
            if ((sIdx >= 0) != (tIdx >= 0))
                return false;

            sIdx--;
            tIdx--;
        }

        return true;
    }
};

//leetcode submit region end(Prohibit modification and deletion)
