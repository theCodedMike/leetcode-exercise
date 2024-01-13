use leetcode_exercise::leetcode::editor::cn::_171_excel_sheet_column_number::Solution;

#[test]
fn excel_sheet_column_number() {
    let column_title = "A".to_string();
    let res = Solution::title_to_number(column_title);
    assert_eq!(res, 1);

    let column_title = "AB".to_string();
    let res = Solution::title_to_number(column_title);
    assert_eq!(res, 28);

    let column_title = "ZY".to_string();
    let res = Solution::title_to_number(column_title);
    assert_eq!(res, 701);
}
