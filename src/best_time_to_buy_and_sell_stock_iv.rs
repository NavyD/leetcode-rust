/// 注意：k临界值、初始化、反向遍历k
pub mod solution_dp {
    /// # 思路
    ///
    /// 如果 k 超过一个临界值，最大收益就不再取决于允许的最大交易次数，而是取决于股票价格数组的长度。
    /// 一个有收益的交易至少需要两天（在前一天买入，在后一天卖出，前提是买入价格低于卖出价格）。
    /// 如果股票价格数组的长度为 n，则有收益的交易的数量最多为 n / 2（整数除法）。因此 k 的临界值是 n / 2。
    /// 如果给定的 k 不小于临界值，即 k >= n / 2，则可以将 k 扩展为正无穷，此时问题等价于情况二
    ///
    /// ```ignore
    /// if k >= n / 2:
    /// dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + prices[i]);
    /// dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] - prices[i]);
    ///
    /// if k < n / 2:
    /// dp[i][k][0] = dp[i - 1][k][0].max(dp[i - 1][k][1] + prices[i]);
    /// dp[i][k][1] = dp[i - 1][k][1].max(dp[i - 1][k - 1][0] - prices[i]);
    /// ```
    ///
    /// 注意：dp[0].len = k + 1，从后遍历k
    ///
    /// 初始化时k>=1开始dp[0][k][1]=-prices[0]，没有设置dp[0][0][1]的原因是k表示交易次数，而交易0次持有1股票
    /// 买入是不可能的与III类似
    ///
    /// 空间优化：去除dp[i]使用dp[k][2]
    ///
    /// 参考：
    ///
    /// * [状态压缩时，关于k是否需要倒序的一点思考](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iv/solution/zhuang-tai-ya-suo-shi-guan-yu-kshi-fou-dao-xu-yao-/)
    /// * [股票问题系列通解（转载翻译）](https://leetcode-cn.com/circle/article/qiAgHn/)
    /// * [Most consistent ways of dealing with the series of stock problems](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/discuss/108870/Most-consistent-ways-of-dealing-with-the-series-of-stock-problems)
    ///
    /// ### Submissions
    ///
    /// date=20210620, mem=2.7, mem_beats=11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/188156204/
    /// 
    /// date=20210628, mem=2.8, mem_beats=10, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/190384717/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
            if prices.is_empty() {
                return 0;
            }
            let (n, k) = (prices.len(), k as usize);
            if k >= n / 2 {
                let mut dp = vec![(0, 0); n];
                // dp[0].0 = 0;
                dp[0].1 = -prices[0];

                for i in 1..n {
                    dp[i].0 = dp[i - 1].0.max(dp[i - 1].1 + prices[i]);
                    dp[i].1 = dp[i - 1].1.max(dp[i - 1].0 - prices[i]);
                }
                dp[n - 1].0
            } else {
                let mut dp = vec![vec![(0, 0); k + 1]; n];
                for i in 1..=k {
                    // dp[0][i].0 = 0;
                    dp[0][i].1 = -prices[0];
                }

                for i in 1..n {
                    for j in (1..=k).rev() {
                        dp[i][j].0 = dp[i - 1][j].0.max(dp[i - 1][j].1 + prices[i]);
                        dp[i][j].1 = dp[i - 1][j].1.max(dp[i - 1][j - 1].0 - prices[i]);
                    }
                }
                dp[n - 1][k].0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::max_profit);
    }

    fn test<F: Fn(i32, Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(2, vec![2, 4, 1]), 2);
        assert_eq!(f(2, vec![3, 2, 6, 5, 0, 3]), 7);
        assert_eq!(f(2, vec![]), 0);
    }
}
