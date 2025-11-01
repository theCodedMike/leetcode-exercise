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


#include <algorithm>
#include <iostream>
#include <numeric>
#include <unordered_map>
using namespace std;

//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    string minWindow(string s, string t) {
        const auto t_map =
            std::accumulate(
                t.cbegin(), t.cend(), unordered_map<char, int>(),[](unordered_map<char, int> map, char c) {
                    ++map[c];
                    return map;
        });
        auto [left, left_when_min, w_len] = std::make_tuple(0, 0, INT_MAX);
        unordered_map<char, int> s_map;

        for (int right = 0; right < s.size(); ++right) {
            s_map[s[right]]++;
            while (smap_contains_tmap(s_map, t_map)) {
                if (w_len > right - left + 1) {
                    w_len = right - left + 1;
                    left_when_min = left;
                }
                s_map[s[left]]--;
                left++;
            }
        }

        return w_len == INT_MAX ? "" : s.substr(left_when_min, w_len);
    }

    static bool smap_contains_tmap(const unordered_map<char, int> &smap, const unordered_map<char, int> &tmap) {
        return std::all_of(tmap.cbegin(), tmap.cend(), [&smap](auto &pair) {
            if (smap.count(pair.first) == 0)
                return false;
            return smap.at(pair.first) >= pair.second;
        });
    }
};

//leetcode submit region end(Prohibit modification and deletion)
