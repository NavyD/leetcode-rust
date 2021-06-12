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
pub mod solution_dp {
    /// # 思路
    /// 
    /// ## 方法1：前i天可获得的最大利润
    /// 
    /// 子问题：定义一个min_price，表示在前i天时出现的最小价格，假设在第i天卖出的利润`prices[i] - min_price`，
    /// 如果之前的利润更大，则使用之前的利润，否则更新为在第i天卖出的利润。problem(i)表示在前i天可获得的最大利润，
    /// 则有`problem(i) = problem(i - 1).max(prices[i] - min_price)`。problem(i)就表示问题结果获取的最大利润
    /// 
    /// dp方程：`dp[i] = dp[i - 1].max(prices[i] - (min_price = min_price.min(prices[i])))`
    /// 
    /// ```
    /// pub fn max_profit(prices: Vec<i32>) -> i32 {
    ///     let (mut min_price, mut dp) = (prices[0], vec![0; prices.len()]);
    ///     for i in 1..prices.len() {
    ///         min_price = min_price.min(prices[i]);
    ///         dp[i] = dp[i - 1].max(prices[i] - min_price);
    ///     }
    ///     *dp.last().unwrap()
    /// }
    /// assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    /// ```
    /// 
    /// ## 方法2：第i天时卖出的最大利润
    /// 
    /// 子问题：problem(i)表示在第i天时卖出的最大利润，problem(i)不能直接作为问题的结果，但是可以统计一个最大值表示 
    /// 前i天的最大利润。`problem(i) = 0.max(problem(i - 1) + prices[i] - prices[i - 1])`，第i天卖出的最大利润可以
    /// 通过累积差实现如：b3 = a3 - a2, b4 = a4 - a3, b5 = a5 - a4, b6 = a6 - a5. b3 + b4 + b5 + b6 = a6 - a2。
    /// `[参考](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/39038/Kadane's-Algorithm-Since-no-one-has-mentioned-about-this-so-far-:)-(In-case-if-interviewer-twists-the-input)/36798)`
    /// 
    /// 参考：
    /// 
    /// * [股票问题（Python3、C++）](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/solution/gu-piao-wen-ti-python3-c-by-z1m/)
    /// * [Kadane's Algorithm - Since no one has mentioned about this so far :) (In case if interviewer twists the input)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/discuss/39038/Kadane's-Algorithm-Since-no-one-has-mentioned-about-this-so-far-%3A)-(In-case-if-interviewer-twists-the-input))
    /// * [暴力解法、动态规划（Java）](https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/solution/bao-li-mei-ju-dong-tai-gui-hua-chai-fen-si-xiang-b/774434)
    /// 
    /// ### Submissions
    /// 
    /// date=20210612, mem=2.8, mem_beats=73, runtime=12, runtime_beats=91, url=https://leetcode-cn.com/submissions/detail/186035718/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let (mut res, mut dp) = (0, vec![0; prices.len()]);
            for i in 1..prices.len() {
                dp[i] = 0.max(dp[i - 1] + prices[i] - prices[i - 1]);
                res = res.max(dp[i])
            }
            res
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    /// 
    /// 优化方法2
    /// 
    /// 这是优化方法1：
    /// 
    /// ```
    /// pub fn max_profit(prices: Vec<i32>) -> i32 {
    ///     let (mut cur, mut min_price) = (0, prices[0]);
    ///     for i in 1..prices.len() {
    ///         min_price = min_price.min(prices[i]);
    ///         cur = cur.max(prices[i] - min_price);
    ///     }
    ///     cur
    /// }
    /// assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    /// ```
    /// 
    /// ### Submissions
    /// 
    /// date=20210612, mem=3, mem_beats=6, runtime=16, runtime_beats=59, url=https://leetcode-cn.com/submissions/detail/186037766/
    pub struct Solution;

    impl Solution {
        pub fn max_profit(prices: Vec<i32>) -> i32 {
            let (mut res, mut pre) = (0, 0);
            for i in 1..prices.len() {
                pre = 0.max(pre + prices[i] - prices[i - 1]);
                res = res.max(pre);
            }
            res
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

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(func(vec![7, 6, 4, 3, 1]), 0);
    }
}
