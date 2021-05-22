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
    /// `dp[amount] = for i in [0, len - 1] if coins[i] <= amount: min(dp[amount], 1 + dp[amount - coins[i]])`
    ///
    /// 下面使用iter api替换内部for循环：
    /// 
    /// ```ignore
    /// pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    ///     let amount = amount as usize;
    ///     let mut dp = vec![None; amount + 1];
    ///     dp[0] = Some(0);
    ///     for i in 1..dp.len() {
    ///         dp[i] = coins
    ///             .iter()
    ///             .map(|coin| i as i32 - *coin)
    ///             .filter(|pre_idx| *pre_idx >= 0 && dp[*pre_idx as usize].is_some())
    ///             .map(|pre_idx| dp[pre_idx as usize].unwrap())
    ///             .fold(dp[i], |min_count, pre_count| {
    ///                 min_count
    ///                     .map(|v| v.min(pre_count + 1))
    ///                     .or(Some(pre_count + 1))
    ///             });
    ///     }
    ///     dp[amount].unwrap_or(-1)
    /// }
    /// ```
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
    /// 
    /// date=20210522, mem=2, mem_beats=86, runtime=16, runtime_beats=40, url=https://leetcode-cn.com/submissions/detail/179790819/
    pub struct Solution;

    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            let amount = amount as usize;
            let mut dp = vec![None; amount + 1];
            // 理解 dp[0] = 0 的合理性，单独一枚硬币如果能够凑出面值，符合最优子结构 
            dp[0] = Some(0);
            for amount in 1..=amount {
                for coin in &coins {
                    let pre_idx = amount as i32 - coin;
                    if pre_idx >= 0 && dp[pre_idx as usize].is_some() {
                        dp[amount] = dp[amount]
                            // 避免 None.min(Some(v)) 返回一个None
                            .or(Some(i32::MAX))
                            .min(dp[pre_idx as usize].map(|v| v + 1))
                    }
                }
            }
            dp[amount].unwrap_or(-1)
        }
    }
}

pub mod solution_bfs {
    /// # 思路
    ///
    /// ![](https://pic.leetcode-cn.com/32128c822b67e7a851e78165e4498d71519c5ba7c1476e60f7d9e8c2df7487b0-%E5%B1%8F%E5%B9%95%E5%BF%AB%E7%85%A7%202020-03-08%2010.33.52.png)
    ///
    /// 参考：
    ///
    /// * [方法二：广度优先遍历](https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/)
    ///
    /// ### Submissions
    ///
    /// date=20210314, mem=2, mem_beats=81, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/155029103/
    pub struct Solution;

    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            if amount == 0 {
                return 0;
            }

            let mut queue = std::collections::VecDeque::new();
            queue.push_back(amount);

            let amount = amount as usize;
            let mut visited = vec![false; amount + 1];
            visited[amount] = true;
            // coins.sort_unstable();

            let mut steps = 1;
            while !queue.is_empty() {
                for _ in 0..queue.len() {
                    let amount = queue.pop_front().unwrap();
                    for coin in &coins {
                        let rest_amount = amount - coin;
                        if rest_amount == 0 {
                            return steps;
                        }
                        if rest_amount < 0 {
                            // break;
                            continue;
                        }
                        if !visited[rest_amount as usize] {
                            queue.push_back(rest_amount);
                            visited[rest_amount as usize] = true;
                        }
                    }
                }
                steps += 1;
            }
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::coin_change);
        test(solution_bfs::Solution::coin_change);
    }

    fn test<F: Fn(Vec<i32>, i32) -> i32>(func: F) {
        assert_eq!(func(vec![1, 2, 5], 11), 3);
        assert_eq!(func(vec![2], 3), -1);
        assert_eq!(func(vec![1], 0), 0);
        assert_eq!(func(vec![1], 1), 1);
        assert_eq!(func(vec![1], 2), 2);
    }
}
