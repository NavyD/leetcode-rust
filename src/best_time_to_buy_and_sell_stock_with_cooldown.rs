pub mod solution_dp {
    /// # 思路
    ///
    /// ```ignore
    /// dp[i][k][0] = max(dp[i-1][k][0], dp[i-1][k][1] + prices[i])
    /// dp[i][k][1] = max(dp[i-1][k][1], dp[i-2][k][1] - prices[i])
    /// ```
    ///
    /// 参考：
    ///
    /// * [股票问题系列通解（转载翻译）](https://leetcode-cn.com/circle/article/qiAgHn/)
    ///
    /// ### Submissions
    ///
    /// date=20210630, mem=2, mem_beats=75, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/190947805/
    ///
    /// date=20210701, mem=2.2, mem_beats=20, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/191325218/
    ///
    /// date=20210715, mem=2, mem_beats=90, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195942618/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let n = prices.len();
            let mut dp = vec![(0, 0); n];
            dp[0].1 = -prices[0];

            for i in 1..n {
                dp[i].0 = dp[i - 1].0.max(dp[i - 1].1 + prices[i]);
                dp[i].1 = dp[i - 1]
                    .1
                    .max(if i == 1 { 0 } else { dp[i - 2].0 } - prices[i]);
            }
            dp[n - 1].0
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [股票问题系列通解（转载翻译）](https://leetcode-cn.com/circle/article/qiAgHn/)
    ///
    /// ### Submissions
    ///
    /// date=20210630, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/190952666/
    ///
    /// date=20210701, mem=2, mem_beats=80, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/191327047/
    ///
    /// date=20210715, mem=2, mem_beats=85, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/195945644/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            if prices.is_empty() {
                return 0;
            }
            let (mut profit0, mut pre_profit0, mut profit1) = (0, 0, -prices[0]);
            for i in 1..prices.len() {
                let next_profit0 = profit0.max(profit1 + prices[i]);
                profit1 = profit1.max(pre_profit0 - prices[i]);
                pre_profit0 = profit0;
                profit0 = next_profit0;
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
        test(solution_dp::Solution::max_profit);
        test(solution_dp_optimized::Solution::max_profit);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![1, 2, 3, 0, 2]), 3);
        assert_eq!(f(vec![1]), 0);
    }
}
