//! Say you have an array for which the ith element is the price of a given stock on day i.
//! 
//! If you were only permitted to complete at most one transaction (i.e., buy one and sell one share of the stock), design an algorithm to find the maximum profit.
//! 
//! Note that you cannot sell a stock before you buy one.
//! 
//! Example 1:
//! 
//! Input: [7,1,5,3,6,4]
//! Output: 5
//! Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
//!              Not 7-1 = 6, as selling price needs to be larger than buying price.
//! Example 2:
//! 
//! Input: [7,6,4,3,1]
//! Output: 0
//! Explanation: In this case, no transaction is done, i.e. max profit = 0.

pub mod solution_dp {
    /// # 思路
    /// 
    /// 设最大profit为dp[n]，max_profit=max_price - min_price
    /// `dp[i] = max(dp[i - 1], max(cur_price - min_price, 0))`
    /// 
    /// ```ignore
    /// prices=[7,1,5,3,6,4]
    /// dp[0] = 0, min_price = 7
    /// dp[1] = max(0, max(1 - 7, 0)) = 0, min_price = 1,
    /// dp[2] = max(0, max(5 - 1, 0)) = 4, min_price = 1,
    /// dp[3] = max(4, max(3 - 1, 0)) = 4, min_price = 1,
    /// dp[4] = max(4, max(6 - 1, 0)) = 5, min_price = 1,
    /// dp[5] = max(5, max(4 - 1, 0)) = 5, min_price = 1,
    /// ```
    /// 
    /// 可以看到dp[i]仅用到dp[i-1]与min_price，可用prev_profit替代dp[]
    /// 
    /// ## Submissions
    /// 
    /// date=20200622, mem=2.3, mem_beats=14.03, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356776388/
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            if prices.is_empty() {
                return 0;
            }
            let (mut min_price, mut prev_profit) = (prices[0], 0);
            for p in prices {
                prev_profit = prev_profit.max((p - min_price).max(0));
                if p < min_price {
                    min_price = p;
                }
            }
            prev_profit
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let prices = vec![7,1,5,3,6,4];
            assert_eq!(Solution::max_profit(prices), 5);

            let prices = vec![7,6,4,3,1];
            assert_eq!(Solution::max_profit(prices), 0);
        }
    }
}