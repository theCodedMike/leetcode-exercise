package leetcode.editor.cn;
//给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。
//
// 
//
// 注意： 
//
// 
// 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。 
// 如果 s 中存在这样的子串，我们保证它是唯一的答案。 
// 
//
// 
//
// 示例 1： 
//
// 
//输入：s = "ADOBECODEBANC", t = "ABC"
//输出："BANC"
//解释：最小覆盖子串 "BANC" 包含来自字符串 t 的 'A'、'B' 和 'C'。
// 
//
// 示例 2： 
//
// 
//输入：s = "a", t = "a"
//输出："a"
//解释：整个字符串 s 是最小覆盖子串。
// 
//
// 示例 3: 
//
// 
//输入: s = "a", t = "aa"
//输出: ""
//解释: t 中两个字符 'a' 均应包含在 s 的子串中，
//因此没有符合条件的子字符串，返回空字符串。 
//
// 
//
// 提示： 
//
// 
// m == s.length 
// n == t.length 
// 1 <= m, n <= 10⁵ 
// s 和 t 由英文字母组成 
// 
//
// 
//进阶：你能设计一个在 
//o(m+n) 时间内解决此问题的算法吗？
//
// Related Topics 哈希表 字符串 滑动窗口 👍 2698 👎 0

import java.util.HashMap;

//leetcode submit region begin(Prohibit modification and deletion)
class _76_最小覆盖子串 {
    public String minWindow(String s, String t) {
        HashMap<Character, Integer> tMap = new HashMap<>();
        for (int i = 0, len = t.length(); i < len; i++) {
            char c = t.charAt(i);
            tMap.put(c, tMap.getOrDefault(c, 0) + 1);
        }
        int left = 0;
        int leftWhenMin = 0;
        int wLen = Integer.MAX_VALUE;
        HashMap<Character, Integer> sMap = new HashMap<>();

        for (int right = 0, len = s.length(); right < len; right++) {
            char c = s.charAt(right);
            sMap.put(c, sMap.getOrDefault(c, 0) + 1);

            while (sMapContainsTmap(sMap, tMap)) {
                if (wLen > (right - left + 1)) {
                    wLen = right - left + 1;
                    leftWhenMin = left;
                }

                char leftC = s.charAt(left);
                sMap.put(leftC, sMap.get(leftC) - 1);
                left++;
            }
        }

        return wLen == Integer.MAX_VALUE ? "" : s.substring(leftWhenMin, leftWhenMin + wLen);
    }

    boolean sMapContainsTmap(HashMap<Character, Integer> sMap, HashMap<Character, Integer> tMap) {
        return tMap.entrySet().stream().allMatch((tEntry) -> {
            if (!sMap.containsKey(tEntry.getKey()))
                return false;
            return sMap.get(tEntry.getKey()) >= tEntry.getValue();
        });
    }
}
//leetcode submit region end(Prohibit modification and deletion)
