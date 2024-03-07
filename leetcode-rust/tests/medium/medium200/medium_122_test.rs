use leetcode_rust::leetcode::editor::cn::_122_best_time_to_buy_and_sell_stock_i_i::Solution;

#[test]
fn best_time_to_buy_and_sell_stock_ii_1() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    assert_eq!(Solution::max_profit(prices), 7);
}

#[test]
fn best_time_to_buy_and_sell_stock_ii_2() {
    let prices = vec![1, 2, 3, 4, 5];
    assert_eq!(Solution::max_profit(prices), 4);
}

#[test]
fn best_time_to_buy_and_sell_stock_ii_3() {
    let prices = vec![7, 6, 4, 3, 1];
    assert_eq!(Solution::max_profit(prices), 0);
}
