use leetcode_rust::leetcode::editor::cn::_7_reverse_integer::Solution;

#[test]
fn reverse_integer() {
    let res = Solution::reverse(123);
    assert_eq!(res, 321);

    let res = Solution::reverse(120);
    assert_eq!(res, 21);

    let res = Solution::reverse(1234567899);
    assert_eq!(res, 0);

    let res = Solution::reverse(-123);
    assert_eq!(res, -321);

    let res = Solution::reverse(-120);
    assert_eq!(res, -21);

    let res = Solution::reverse(-1234567899);
    assert_eq!(res, 0);
}
