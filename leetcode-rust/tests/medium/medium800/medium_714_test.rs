use leetcode_exercise::leetcode::editor::cn::_714_best_time_to_buy_and_sell_stock_with_transaction_fee::Solution;

#[test]
fn best_time_to_buy_and_sell_stock_with_transaction_fee_1() {
    let prices = vec![1, 3, 2, 8, 4, 9];
    let fee = 2;
    assert_eq!(Solution::max_profit(prices, fee), 8);
}

#[test]
fn best_time_to_buy_and_sell_stock_with_transaction_fee_2() {
    let prices = vec![1, 3, 7, 5, 10, 3];
    let fee = 3;
    assert_eq!(Solution::max_profit(prices, fee), 6);
}
