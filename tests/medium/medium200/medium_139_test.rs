use leetcode_exercise::leetcode::editor::cn::_139_word_break::Solution;

#[test]
fn word_break() {
    let s = "leetcode".to_string();
    let word_dict = vec!["leet".to_string(), "code".to_string()];
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res, true);

    let s = "applepenapple".to_string();
    let word_dict = vec!["apple".to_string(), "pen".to_string()];
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res, true);

    let s = "catsandog".to_string();
    let word_dict = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res, false);

    let s = "cars".to_string();
    let word_dict = vec!["car".to_string(), "ca".to_string(), "rs".to_string()];
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res, true);
}

///
/// Time Limit Exceeded
///
#[test]
fn word_break2() {
    let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
    let word_dict = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
        "aaaaa".to_string(),
        "aaaaaa".to_string(),
        "aaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
        "aaaaaaaaa".to_string(),
        "aaaaaaaaaa".to_string(),
    ];
    let res = Solution::word_break(s, word_dict);
    assert_eq!(res, true);
}
