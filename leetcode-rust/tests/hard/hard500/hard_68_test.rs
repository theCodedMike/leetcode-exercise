use leetcode_rust::leetcode::editor::cn::_68_text_justification::Solution;

#[test]
fn text_justification() {
    let words = vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string(),
    ];
    let max_width = 16;
    let res = Solution::full_justify(words, max_width);
    assert_eq!(
        res,
        ["This    is    an", "example  of text", "justification.  "]
    );
}

#[test]
fn text_justification2() {
    let words = vec![
        "What".to_string(),
        "must".to_string(),
        "be".to_string(),
        "acknowledgment".to_string(),
        "shall".to_string(),
        "be".to_string(),
    ];
    let max_width = 16;
    let res = Solution::full_justify(words, max_width);
    assert_eq!(
        res,
        ["What   must   be", "acknowledgment  ", "shall be        "]
    );
}

#[test]
fn text_justification3() {
    let words = vec![
        "My".to_string(),
        "momma".to_string(),
        "always".to_string(),
        "said,".to_string(),
        "\"Life".to_string(),
        "was".to_string(),
        "like".to_string(),
        "a".to_string(),
        "box".to_string(),
        "of".to_string(),
        "chocolates.".to_string(),
        "You".to_string(),
        "never".to_string(),
        "know".to_string(),
        "what".to_string(),
        "you're".to_string(),
        "gonna".to_string(),
        "get.".to_string(),
    ];
    let max_width = 20;
    let res = Solution::full_justify(words, max_width);
    assert_eq!(
        res,
        [
            "My    momma   always",
            "said, \"Life was like",
            "a box of chocolates.",
            "You  never know what",
            "you're gonna get.   "
        ]
    );
}
