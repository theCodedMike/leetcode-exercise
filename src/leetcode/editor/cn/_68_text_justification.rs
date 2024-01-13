//Given an array of strings words and a width maxWidth, format the text such
//that each line has exactly maxWidth characters and is fully (left and right)
//justified.
//
// You should pack your words in a greedy approach; that is, pack as many words
//as you can in each line. Pad extra spaces ' ' when necessary so that each line
//has exactly maxWidth characters.
//
// Extra spaces between words should be distributed as evenly as possible. If
//the number of spaces on a line does not divide evenly between words, the empty
//slots on the left will be assigned more spaces than the slots on the right.
//
// For the last line of text, it should be left-justified, and no extra space
//is inserted between words.
//
// Note:
//
//
// A word is defined as a character sequence consisting of non-space characters
//only.
// Each word's length is guaranteed to be greater than 0 and not exceed
//maxWidth.
// The input array words contains at least one word.
//
//
//
// Example 1:
//
//
//Input: words = ["This", "is", "an", "example", "of", "text", "justification."]
//, maxWidth = 16
//Output:
//[
//   "This    is    an",
//   "example  of text",
//   "justification.  "
//]
//
// Example 2:
//
//
//Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth =
//16
//Output:
//[
//  "What   must   be",
//  "acknowledgment  ",
//  "shall be        "
//]
//Explanation: Note that the last line is "shall be    " instead of "shall
//be", because the last line must be left-justified instead of fully-justified.
//Note that the second line is also left-justified because it contains only one
//word.
//
// Example 3:
//
//
//Input: words = ["Science","is","what","we","understand","well","enough","to",
//"explain","to","a","computer.","Art","is","everything","else","we","do"],
//maxWidth = 20
//Output:
//[
//  "Science  is  what we",
//  "understand      well",
//  "enough to explain to",
//  "a  computer.  Art is",
//  "everything  else  we",
//  "do                  "
//]
//
//
// Constraints:
//
//
// 1 <= words.length <= 300
// 1 <= words[i].length <= 20
// words[i] consists of only English letters and symbols.
// 1 <= maxWidth <= 100
// words[i].length <= maxWidth
//
//
// Related Topics Array String Simulation 👍 2440 👎 3563

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut res = vec![];
        let mut iter = words.into_iter().peekable();

        let mut temp = vec![];
        let mut sub_len = 0;
        while let Some(cur) = iter.next() {
            sub_len += cur.len() as i32;
            temp.push(cur);

            match iter.peek() {
                None => {
                    Solution::handle_sub_words(&mut temp, &mut res, max_width, true);
                }
                Some(next) => {
                    if sub_len as usize + 1 + next.len() > max_width {
                        Solution::handle_sub_words(&mut temp, &mut res, max_width, false);
                        sub_len = -1;
                    }
                }
            }

            sub_len += 1;
        }

        res
    }
    fn handle_sub_words(
        words: &mut Vec<String>,
        res: &mut Vec<String>,
        max_width: usize,
        is_last_line: bool,
    ) {
        if words.is_empty() {
            return;
        }
        let space_count = words.len() - 1;
        let total_len = words.iter().map(|w| w.len()).sum::<usize>();
        let space_left = max_width - total_len;
        let mut new_word = "".to_string();
        if space_count == 0 {
            new_word.push_str(&words[0]);
            new_word.push_str(&" ".repeat(space_left));
        } else if is_last_line {
            new_word = words.join(" ");
            new_word.push_str(&" ".repeat(space_left - space_count));
        } else {
            let rem = space_left % space_count;
            let quo = space_left / space_count;
            if rem == 0 {
                new_word = words.join(&" ".repeat(quo));
            } else {
                for (idx, word) in words.iter().enumerate() {
                    new_word.push_str(word);
                    if idx == space_count {
                        break;
                    }
                    if idx < rem {
                        new_word.push_str(&" ".repeat(quo + 1));
                    } else {
                        new_word.push_str(&" ".repeat(quo));
                    }
                }
            }
        }

        res.push(new_word);
        words.clear();
    }
}
//leetcode submit region end(Prohibit modification and deletion)
