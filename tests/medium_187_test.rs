use leetcode_exercise::leetcode::editor::en::_187_repeated_d_n_a_sequences::Solution;

#[test]
fn repeated_dna_sequences() {
    let s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string();
    let res = Solution::find_repeated_dna_sequences(s);
    assert_eq!(res, ["AAAAACCCCC", "CCCCCAAAAA"]);

    let s = "AAAAAAAAAAA".to_string();
    let res = Solution::find_repeated_dna_sequences(s);
    assert_eq!(res, ["AAAAAAAAAA"]);
}
