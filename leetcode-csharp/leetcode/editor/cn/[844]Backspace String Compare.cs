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


//leetcode submit region begin(Prohibit modification and deletion)

public class Solution
{
    public bool BackspaceCompare(string s, string t)
    {
        //return BuildString(s, t);
        return TwoPointers(s, t);
    }

    bool BuildString(string s, string t)
    {
        Func<string, string> build = (str) =>
        {
            Stack<char> res = new Stack<char>();

            foreach (var ch in str.ToCharArray())
            {
                if (ch == '#')
                {
                    if (res.Count != 0)
                        res.Pop();
                }
                else
                    res.Push(ch);
            }

            return new string(res.ToArray());
        };

        return build(s) == build(t);
    }
    
    bool TwoPointers(string s, string t)
    {
        (int sIdx, int tIdx) = (s.Length - 1, t.Length - 1);
        (int sSharpCount, int tSharpCount) = (0, 0);

        while (sIdx >= 0 || tIdx >= 0)
        {
            while (sIdx >= 0)
            {
                if (s[sIdx] == '#')
                {
                    sSharpCount++;
                    sIdx--;
                } else if (sSharpCount > 0)
                {
                    sSharpCount--;
                    sIdx--;
                }
                else
                    break;
            }
            while (tIdx >= 0)
            {
                if (t[tIdx] == '#')
                {
                    tSharpCount++;
                    tIdx--;
                } else if (tSharpCount > 0)
                {
                    tSharpCount--;
                    tIdx--;
                }
                else
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
}
//leetcode submit region end(Prohibit modification and deletion)