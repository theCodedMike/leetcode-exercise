//Given two strings s and t of lengths m and n respectively, return the minimum 
//window substring of s such that every character in t (including duplicates) is 
//included in the window. If there is no such substring, return the empty string 
//"". 
//
// The testcases will be generated such that the answer is unique. 
//
// 
// Example 1: 
//
// 
//Input: s = "ADOBECODEBANC", t = "ABC"
//Output: "BANC"
//Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' 
//from string t.
// 
//
// Example 2: 
//
// 
//Input: s = "a", t = "a"
//Output: "a"
//Explanation: The entire string s is the minimum window.
// 
//
// Example 3: 
//
// 
//Input: s = "a", t = "aa"
//Output: ""
//Explanation: Both 'a's from t must be included in the window.
//Since the largest window of s only has one 'a', return empty string.
// 
//
// 
// Constraints: 
//
// 
// m == s.length 
// n == t.length 
// 1 <= m, n <= 10⁵ 
// s and t consist of uppercase and lowercase English letters. 
// 
//
// 
// Follow up: Could you find an algorithm that runs in O(m + n) time? 
//
// Related Topics 哈希表 字符串 滑动窗口 👍 3422 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public string MinWindow(string s, string t)
    {
        Dictionary<char, int> tMap = new Dictionary<char, int>();
        foreach (var c in t)
            if (!tMap.TryAdd(c, 1))
                tMap[c]++;
        (int left, int leftWhenMin, int wLen) = (0, 0, Int32.MaxValue);
        Dictionary<char, int> sMap = new Dictionary<char, int>();
        
        for (int right = 0; right < s.Length; right++)
        {
            if (!sMap.TryAdd(s[right], 1))
                sMap[s[right]]++;
            
            while (SmapContainsTmap(sMap, tMap))
            {
                if (wLen > right - left + 1)
                {
                    wLen = right - left + 1;
                    leftWhenMin = left;
                }
                sMap[s[left]]--;
                left++;
            }
        }

        return wLen == Int32.MaxValue ? "" : s.Substring(leftWhenMin, wLen);
    }
    
    bool SmapContainsTmap(Dictionary<char, int> sMap, Dictionary<char, int> tMap)
    {
        return tMap.All(pair =>
        {
            if (!sMap.TryGetValue(pair.Key, out var sVal))
                return false;
            return sVal >= pair.Value;
        });
    }
}
//leetcode submit region end(Prohibit modification and deletion)
