use leetcode_exercise::leetcode::editor::cn::_43_multiply_strings::Solution;

#[test]
fn multiply_strings() {
    let num1 = "498828660196".to_string();
    let num2 = "840477629533".to_string();

    let product = Solution::multiply(num1, num2);

    assert_eq!(product, "419254329864656431168468");
}
