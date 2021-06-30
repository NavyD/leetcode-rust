/// 第一次写：菜
///
/// ```ignore
/// let mut min = prices[0];
/// let mut max = -1;
/// let mut profit = 0;
/// for price in prices {
///     if max < 0 {
///         if min > price {
///             min = price;
///             max = price;
///         }
///     } else if max < price {
///         profit += price - min;
///         max = -1;
///     }
/// }
/// ```
///
/// 问题：为何在optimized中可以直接使用之前改变的变量但是结果与分开时相同
///
/// ```ignore
/// // 在profit1计算时被之前的profit0影响
/// profit0 = profit0.max(profit1 + prices[i]);
/// profit1 = profit1.max(profit0 - prices[i]);
///
/// // 分开变量影响
/// let newProfit1 = profit1.max(profit0 - prices[i]);
/// let newProfit0 = profit0.max(profit1 + prices[i]);
/// profit0 = newProfit0;
/// profit1 = newProfit1;
/// ```
///
pub mod solution_greedy {
    /// # 思路
    ///
    /// 贪心算法的直觉：由于不限制交易次数，只要今天股价比昨天高，就交易
    ///
    /// 参考：
    ///
    /// - [暴力搜索、贪心算法、动态规划（Java）](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-ii/solution/tan-xin-suan-fa-by-liweiwei1419-2/)
    ///
    /// ### Submissions
    ///
    /// date=20210112, mem=2.1, mem_beats=86, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/137794557/
    ///
    /// date=20210113, mem=2.1, mem_beats=73, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/138054633/
    ///
    /// date=20210124, mem=2.2, mem_beats=40, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140733456/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut max_profit = 0;
            for i in 1..prices.len() {
                let profit = prices[i] - prices[i - 1];
                if profit > 0 {
                    max_profit += profit;
                }
            }
            max_profit
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// ```ignore
    /// T[i][k][0] = max(T[i - 1][k][0], T[i - 1][k][1] + prices[i])
    /// T[i][k][1] = max(T[i - 1][k][1], T[i - 1][k - 1][0] - prices[i]) = max(T[i - 1][k][1], T[i - 1][k][0] - prices[i])
    /// ```
    ///
    /// 参考：
    ///
    /// * [股票问题系列通解（转载翻译）](https://leetcode-cn.com/circle/article/qiAgHn/)
    /// * [Most consistent ways of dealing with the series of stock problems](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/108870/Most-consistent-ways-of-dealing-with-the-series-of-stock-problems)
    ///
    /// ### Submissions
    ///
    /// date=20210618, mem=3.4, mem_beats=5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/187704657/
    ///
    /// date=20210619, mem=2.2, mem_beats=21, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/187925260/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let n = prices.len();
            let mut dp = vec![(0, 0); n];
            dp[0].0 = 0;
            dp[0].1 = -prices[0];

            for i in 1..n {
                dp[i].0 = dp[i - 1].0.max(dp[i - 1].1 + prices[i]);
                dp[i].1 = dp[i - 1].1.max(dp[i - 1].0 - prices[i]);
            }
            dp[n - 1].0
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// ### Submissions
    ///
    /// date=20210618, mem=2.1, mem_beats=88, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/187704978/
    ///
    /// date=20210619, mem=2.2, mem_beats=40, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/187925686/
    ///
    /// date=20210630, mem=2.2, mem_beats=75, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/190959228/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let (mut profit0, mut profit1) = (0, -prices[0]);
            for i in 1..prices.len() {
                let next0 = profit0.max(profit1 + prices[i]);
                let next1 = profit1.max(profit0 - prices[i]);
                profit0 = next0;
                profit1 = next1;
            }
            profit0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::max_profit);
        test(solution_dp::Solution::max_profit);
        test(solution_dp_optimized::Solution::max_profit);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(func(vec![7, 6, 4, 3, 1]), 0);
    }
}
