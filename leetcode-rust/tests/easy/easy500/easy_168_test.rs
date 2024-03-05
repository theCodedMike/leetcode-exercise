use leetcode_exercise::leetcode::editor::cn::_168_excel_sheet_column_title::Solution;

#[test]
fn excel_sheet_column_title() {
    let column_number = 1;
    let res = Solution::convert_to_title(column_number);
    assert_eq!(res, "A");

    let column_number = 28;
    let res = Solution::convert_to_title(column_number);
    assert_eq!(res, "AB");

    let column_number = 701;
    let res = Solution::convert_to_title(column_number);
    assert_eq!(res, "ZY");

    let column_number = 52;
    let res = Solution::convert_to_title(column_number);
    assert_eq!(res, "AZ");
}
