/// 注意交易次数在买入`-prices[i]`要算作一次交易，从k-1算起：只有买入操作会改变允许的最大交易次数。
/// 下面是错误的：
/// ```ignore
/// dp[i][0][0] = dp[i - 1][0][0].max(dp[i - 1][0][1] + prices[i])
/// dp[i][0][1] = dp[i - 1][0][1].max(dp[i - 1][0][0] - prices[i])
/// dp[i][1][0] = dp[i - 1][1][0].max(dp[i - 1][1][1] + prices[i])
/// dp[i][1][0] = dp[i - 1][1][1].max(dp[i - 1][1][0] - prices[i])
/// ```
pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        /// # 思路
        ///
        /// ```ignore
        /// T[i][2][0] = max(T[i - 1][2][0], T[i - 1][2][1] + prices[i])
        /// T[i][2][1] = max(T[i - 1][2][1], T[i - 1][1][0] - prices[i])
        /// T[i][1][0] = max(T[i - 1][1][0], T[i - 1][1][1] + prices[i])
        /// T[i][1][1] = max(T[i - 1][1][1], T[i - 1][0][0] - prices[i]) = max(T[i - 1][1][1], -prices[i])
        /// ```
        ///
        /// 参考：
        ///
        /// * [股票问题系列通解（转载翻译）](https://leetcode-cn.com/circle/article/qiAgHn/)
        /// * [Most consistent ways of dealing with the series of stock problems](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/108870/Most-consistent-ways-of-dealing-with-the-series-of-stock-problems)
        ///
        /// ### Submissions
        ///
        /// date=20210619, mem=21.5, mem_beats=7, runtime=140, runtime_beats=7, url=https://leetcode-cn.com/submissions/detail/187916639/
        ///
        /// date=20210620, mem=21.3, mem_beats=7, runtime=140, runtime_beats=7, url=https://leetcode-cn.com/submissions/detail/188161320/
        ///
        /// date=20210706, mem=7.3, mem_beats=16, runtime=36, runtime_beats=16, url=https://leetcode-cn.com/submissions/detail/192800162/
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let n = prices.len();
            let mut dp = vec![vec![(0, 0); 3]; n];
            dp[0][1].1 = -prices[0];
            dp[0][2].1 = -prices[0];

            for i in 1..n {
                dp[i][1].0 = dp[i - 1][1].0.max(dp[i - 1][1].1 + prices[i]);
                dp[i][1].1 = dp[i - 1][1].1.max(dp[i - 1][0].0 - prices[i]);

                dp[i][2].0 = dp[i - 1][2].0.max(dp[i - 1][2].1 + prices[i]);
                dp[i][2].1 = dp[i - 1][2].1.max(dp[i - 1][1].0 - prices[i]);
            }
            dp[n - 1][2].0
        }
    }
}

pub mod solution_dp_optimized {

    /// # 思路
    ///
    /// 注意顺序对profit_10计算前后对profit21没有影响
    ///
    /// ### Submissions
    ///
    /// date=20210619, mem=2.8, mem_beats=100, runtime=12, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/187918920/
    ///
    /// date=20210620, mem=3, mem_beats=57, runtime=16, runtime_beats=35, url=https://leetcode-cn.com/submissions/detail/188162387/
    ///
    /// date=20210706, mem=2.8, mem_beats=88, runtime=16, runtime_beats=44, url=https://leetcode-cn.com/submissions/detail/192800961/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let (mut profit10, mut profit11) = (0, -prices[0]);
            let (mut profit20, mut profit21) = (0, -prices[0]);

            for i in 1..prices.len() {
                profit10 = profit10.max(profit11 + prices[i]);
                profit11 = profit11.max(-prices[i]);

                profit20 = profit20.max(profit21 + prices[i]);
                profit21 = profit21.max(profit10 - prices[i]);
            }
            profit20
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::max_profit);
        test(solution_dp_optimized::Solution::max_profit);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(f(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(f(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(f(vec![1]), 0);
    }
}
