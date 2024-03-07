use leetcode_rust::leetcode::editor::cn::_166_fraction_to_recurring_decimal::Solution;

#[test]
fn fraction_to_recurring_decimal() {
    let numerator = 1;
    let denominator = 2;
    let res = Solution::fraction_to_decimal(numerator, denominator);
    assert_eq!(res, "0.5");

    let numerator = 2;
    let denominator = 1;
    let res = Solution::fraction_to_decimal(numerator, denominator);
    assert_eq!(res, "2");

    let numerator = 4;
    let denominator = 333;
    let res = Solution::fraction_to_decimal(numerator, denominator);
    assert_eq!(res, "0.(012)");
}
