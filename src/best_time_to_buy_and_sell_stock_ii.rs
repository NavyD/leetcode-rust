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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_greedy::Solution::max_profit);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(func(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(func(vec![7, 6, 4, 3, 1]), 0);
    }
}
