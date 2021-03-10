pub mod solution_dp {
    /// # 思路
    ///
    /// 设`dp[i]`表示凑齐总价值 i 需要的最少硬币个数
    ///
    /// 对于`coins = [1, 2, 5], amount = 11`, 凑成面值为 11 的最少硬币个数可以由以下三者的最小值得到：
    ///
    /// - 凑成面值为 10 的最少硬币个数 + 面值为 1 的这一枚硬币；
    /// - 凑成面值为 9 的最少硬币个数 + 面值为 2 的这一枚硬币；
    /// - 凑成面值为 6 的最少硬币个数 + 面值为 5 的这一枚硬币。
    ///
    /// 即: `dp[11] = min(dp[10] + 1, dp[9] + 1, dp[6] + 1)`,在前一个最小硬币数+当前1个 硬币面值`coins[..]`取最小值
    ///
    /// 即: `if i - coins[j] >= 0 { dp[i] = min(dp[i], dp[i - coins[j]] + 1) }`
    ///
    /// 参考:
    ///
    /// * [动态规划、完全背包、BFS（包含完全背包问题公式推导）](https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/)
    ///
    /// ### Submissions
    ///
    /// date=20210309, mem=2.1, mem_beats=50, runtime=16, runtime_beats=35, url=https://leetcode-cn.com/submissions/detail/152889099/
    /// 
    /// date=20210310, mem=2.1, mem_beats=50, runtime=20, runtime_beats=35, url=https://leetcode-cn.com/submissions/detail/153326263/
    pub struct Solution;

    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            let amount = amount as usize;
            let mut dp = vec![None; amount + 1];
            dp[0] = Some(0);
            for i in 1..dp.len() {
                dp[i] = coins
                    .iter()
                    .map(|coin| i as i32 - *coin)
                    .filter(|pre_idx| *pre_idx >= 0 && dp[*pre_idx as usize].is_some())
                    .map(|pre_idx| dp[pre_idx as usize].unwrap())
                    .fold(dp[i], |min_count, pre_count| {
                        min_count
                            .map(|v| v.min(pre_count + 1))
                            .or(Some(pre_count + 1))
                    });
            }
            dp[amount].unwrap_or(-1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::coin_change);
    }

    fn test<F: Fn(Vec<i32>, i32) -> i32>(func: F) {
        assert_eq!(func(vec![1, 2, 5], 11), 3);
        assert_eq!(func(vec![2], 3), -1);
        assert_eq!(func(vec![1], 0), 0);
        assert_eq!(func(vec![1], 1), 1);
        assert_eq!(func(vec![1], 2), 2);
    }
}
