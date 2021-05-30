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
/// 再次：定义了problem(i)为第i天的最大利润，但是在比较利润时使用了`prices[i - 1] - min_price`
/// 就离谱。`problem(i) = problem(i-1).max(prices[i] - min_price).max(0)`，应该是
/// `prices[i] - prices[i-1]`，还量没有注意为第i天卖出可以获得的最大利润，要考虑
/// i天是否卖出与买入
///
/// ```ignore
///  /// problem(i) = problem(i-1).max(prices[i] - min_price).max(0)
/// pub fn max_profit(prices: Vec<i32>) -> i32 {
///     let (mut min_price, mut res) = (prices[0], 0);
///     let mut dp = vec![0; prices.len() + 1];
///     for i in 1..dp.len() {
///         let price = prices[i - 1];
///         dp[i] = dp[i - 1].max(prices[i - 1] - min_price);
///         if min_price > price {
///             dp[i] = 0;
///             min_price = min_price.min(price);
///         }
///     }
///     *dp.last().unwrap()
/// }
/// ```
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
    /// ## 1
    /// 
    /// 子问题：定义一个min_price，表示在前i天时出现的最小价格，假设在第i天卖出的利润`prices[i] - min_price`，
    /// 如果之前的利润更大，则使用之前的利润，否则更新为在第i天卖出的利润。problem(i)表示在前i天可获得的最大利润，
    /// 则有`problem(i) = problem(i - 1).max(prices[i] - min_price)`。problem(i)就表示问题结果获取的最大利润
    /// 
    /// dp方程：`dp[i] = dp[i - 1].max(prices[i] - (min_price = min_price.min(prices[i])))`
    /// 
    /// 
    /// 
    /// ## 2
    /// 
    /// 子问题：problem(i)表示在第i天时卖出的最大利润，problem(i)不能直接作为问题的结果，但是可以统计一个最大值表示 
    /// 前i天的最大利润。`problem(i) = 0.max(problem(i - 1) + prices[i] - prices[i - 1])`
    /// 
    /// 参考：
    /// 
    /// * [股票问题（Python3、C++）](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/solution/gu-piao-wen-ti-python3-c-by-z1m/)
    /// * [Kadane's Algorithm - Since no one has mentioned about this so far :) (In case if interviewer twists the input)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/39038/Kadane's-Algorithm-Since-no-one-has-mentioned-about-this-so-far-%3A)-(In-case-if-interviewer-twists-the-input))
    /// * [暴力解法、动态规划（Java）](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/solution/bao-li-mei-ju-dong-tai-gui-hua-chai-fen-si-xiang-b/774434)
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let mut min_price = prices[0];
            let mut dp = prices;
            dp[0] = 0;
            for i in 1..dp.len() {
                let price = dp[i];
                min_price = min_price.min(price);
                dp[i] = dp[i - 1].max(price - min_price);
            }
            *dp.last().unwrap()
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
        test(max_profit);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(func(vec![7, 6, 4, 3, 1]), 0);
    }
    /// problem(i) = problem(i-1).max(problem(i-1) + prices[i] - prices[i-1]).max(0)
    ///
    /// 
    /// dp[i] = dp[i - 1].max(prices[i] - min_price=min_price.min(prices[i]))
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![0; prices.len()];
        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];
            dp[i] = 0.max(dp[i - 1] + diff);
            res = res.max(dp[i]);
        }
        res
    }
}
