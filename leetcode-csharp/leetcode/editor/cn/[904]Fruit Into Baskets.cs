//You are visiting a farm that has a single row of fruit trees arranged from 
//left to right. The trees are represented by an integer array fruits where fruits[i]
// is the type of fruit the iᵗʰ tree produces. 
//
// You want to collect as much fruit as possible. However, the owner has some 
//strict rules that you must follow: 
//
// 
// You only have two baskets, and each basket can only hold a single type of 
//fruit. There is no limit on the amount of fruit each basket can hold. 
// Starting from any tree of your choice, you must pick exactly one fruit from 
//every tree (including the start tree) while moving to the right. The picked 
//fruits must fit in one of your baskets. 
// Once you reach a tree with fruit that cannot fit in your baskets, you must 
//stop. 
// 
//
// Given the integer array fruits, return the maximum number of fruits you can 
//pick. 
//
// 
// Example 1: 
//
// 
//Input: fruits = [1,2,1]
//Output: 3
//Explanation: We can pick from all 3 trees.
// 
//
// Example 2: 
//
// 
//Input: fruits = [0,1,2,2]
//Output: 3
//Explanation: We can pick from trees [1,2,2].
//If we had started at the first tree, we would only pick from trees [0,1].
// 
//
// Example 3: 
//
// 
//Input: fruits = [1,2,3,2,2]
//Output: 4
//Explanation: We can pick from trees [2,3,2,2].
//If we had started at the first tree, we would only pick from trees [1,2].
// 
//
// 
// Constraints: 
//
// 
// 1 <= fruits.length <= 10⁵ 
// 0 <= fruits[i] < fruits.length 
// 
//
// Related Topics 数组 哈希表 滑动窗口 👍 805 👎 0


//leetcode submit region begin(Prohibit modification and deletion)
public class Solution {
    public int TotalFruit(int[] fruits)
    {
        //return BruteForce(fruits);
        return SlidingWindow(fruits);
    }
    
    int BruteForce(int[] fruits)
    {
        HashSet<int> set = new HashSet<int>();

        for (int width = fruits.Length; width >= 2; --width)
        {
            int left = 0;
            int right = width;

            while (right <= fruits.Length)
            {
                for (int i = left; i < right; ++i)
                {
                    set.Add(fruits[i]);
                    if (set.Count > 2)
                        break;
                }

                if (set.Count == 2)
                    return width;
                ++left;
                right = left + width;
                set.Clear();
            }
        }

        return fruits.Length;
    }

    int SlidingWindow(int[] fruits)
    {
        int res = 0;
        int left = 0;
        Dictionary<int, int> map = new Dictionary<int, int>();

        for (int right = 0; right < fruits.Length; right++)
        {
            if (map.TryGetValue(fruits[right], out var count))
                map[fruits[right]] = count + 1;
            else
                map[fruits[right]] = 1;
            
            while (map.Count > 2)
            {
                map[fruits[left]]--;
                if (map[fruits[left]] == 0)
                    map.Remove(fruits[left]);
                ++left;
            }

            res = Math.Max(res, right - left + 1);
        }
        
        return res;
    }
}
//leetcode submit region end(Prohibit modification and deletion)
