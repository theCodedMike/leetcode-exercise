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


#include <vector>
#include <unordered_set>
#include <unordered_map>
using namespace std;
//leetcode submit region begin(Prohibit modification and deletion)
class Solution {
public:
    int totalFruit(vector<int>& fruits) {
        //return bruteForce(fruits);
        return slidingWindow(fruits);
    }

    int bruteForce(vector<int>& fruits) {
        auto size = fruits.size();
        unordered_set<int> set;

        for (auto width = size; width >= 2; --width) {
            auto left = 0;
            auto right = width;

            while (right <= size) {
                for (auto i = left; i < right; ++i) {
                    set.insert(fruits[i]);
                    if (set.size() > 2)
                        break;
                }
                if (set.size() == 2)
                    return width;

                ++left;
                right = left + width;
                set.clear();
            }
        }

        return size;
    }

    int slidingWindow(vector<int>& fruits) {
        int res = 0;
        int left = 0;
        unordered_map<int, int> map;

        for (auto right = 0; right < fruits.size(); ++right) {
            map[fruits[right]]++;

            while (map.size() > 2) {
                map[fruits[left]]--;
                if (map[fruits[left]] == 0)
                    map.erase(fruits[left]);
                ++left;
            }

            res = std::max(res, right - left + 1);
        }

        return res;
    }
};
//leetcode submit region end(Prohibit modification and deletion)

