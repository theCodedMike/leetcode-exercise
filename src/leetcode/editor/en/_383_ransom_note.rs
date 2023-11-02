//Given two strings ransomNote and magazine, return true if ransomNote can be 
//constructed by using the letters from magazine and false otherwise. 
//
// Each letter in magazine can only be used once in ransomNote. 
//
// 
// Example 1: 
// Input: ransomNote = "a", magazine = "b"
//Output: false
// 
// Example 2: 
// Input: ransomNote = "aa", magazine = "ab"
//Output: false
// 
// Example 3: 
// Input: ransomNote = "aa", magazine = "aab"
//Output: true
// 
// 
// Constraints: 
//
// 
// 1 <= ransomNote.length, magazine.length <= 10⁵ 
// ransomNote and magazine consist of lowercase English letters. 
// 
//
// Related Topics Hash Table String Counting 👍 4598 👎 465

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let a_u8 = b'a';
        let mut arr = magazine.chars().fold([0; 26], |mut arr, c| {
            let i = (c as u8 - a_u8) as usize;
            arr[i] += 1;
            arr
        });

        for c in ransom_note.chars() {
            let i = (c as u8 - a_u8) as usize;
            arr[i] -= 1;
            if arr[i] < 0 {
                return false;
            }
        }

        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)
