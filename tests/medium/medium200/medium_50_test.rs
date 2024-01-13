use leetcode_exercise::leetcode::editor::cn::_50_pow_x_n::Solution;

/// cargo test pow_x_n
#[test]
fn pow_x_n() {
    let x = 2.00000;
    let n = 10;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 1024.00000);

    let x = 2.10000;
    let n = 3;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 9.261000000000001); // 9.26100

    let x = 2.00000;
    let n = -2;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 0.25000);

    let x = 0.00001;
    let n = 2147483647;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 0.0);

    let x = 1.00000;
    let n = 2147483647;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 1.0);

    let x = 2.00000;
    let n = -2147483647;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 0.0);

    let x = 2.00000;
    let n = -2147483648;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 0.0);

    let x = 1.0000000000001;
    let n = -2147483648;
    let res = Solution::my_pow(x, n);
    assert_eq!(res, 0.9997854463000634); // 0.99979
}
