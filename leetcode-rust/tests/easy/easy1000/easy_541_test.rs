use leetcode_rust::leetcode::editor::cn::_541_reverse_string_i_i::Solution;

#[test]
fn reverse_string_ii_1() {
    let s = "abcdefg".to_string();
    let k = 2;
    let res = Solution::reverse_str(s, k);
    assert_eq!(res, "bacdfeg");
}

#[test]
fn reverse_string_ii_2() {
    let s = "abcd".to_string();
    let k = 2;
    let res = Solution::reverse_str(s, k);
    assert_eq!(res, "bacd");
}

#[test]
fn reverse_string_ii_3() {
    let s = "a".to_string();
    let k = 2;
    let res = Solution::reverse_str(s, k);
    assert_eq!(res, "a");
}

#[test]
fn reverse_string_ii_4() {
    let s = "abcdefg".to_string();
    let k = 3;
    let res = Solution::reverse_str(s, k);
    assert_eq!(res, "cbadefg");
}
