pub mod solution_dp {
    /// # 思路
    ///
    /// 对于`coins = [1, 2, 5], amount = 11`, 凑成面值为 11 的最少硬币个数可以由以下三者的最小值得到：
    ///
    /// - 凑成面值为 10 的最少硬币个数 + 面值为 1 的这一枚硬币；
    /// - 凑成面值为 9 的最少硬币个数 + 面值为 2 的这一枚硬币；
    /// - 凑成面值为 6 的最少硬币个数 + 面值为 5 的这一枚硬币。
    ///
    /// 子问题：设`problem(i)`表示凑齐总价值 i 需要的硬币个数，`problem(i) = problem(i - coin) + 1 for coin in coins`
    ///
    /// 即: `dp[11] = min(dp[10] + 1, dp[9] + 1, dp[6] + 1)`,在前一个最小硬币数+当前1个 硬币面值`coins[..]`取最小值
    /// 即: `if i - coins[j] >= 0 { dp[i] = min(dp[i], dp[i - coins[j]] + 1) }`
    ///
    /// dp方程：`dp[amount] = min(dp[amount], 1 + dp[amount - coins[i]]) for coin in coins if coin <= amount`
    ///
    /// 初始化：当amount=1时，dp[1] = coins[0] + dp[0] = 1 + 0，即dp[0]=Some(0)，其它初始化为None
    ///
    /// 注意：
    ///
    /// - mark不能设置为i32::MAX，将会导致dp[i]=i32::MAX+1=-2147483648.min(xx)被设置为-2147483648
    ///
    /// - 有[动态规划、完全背包、BFS（包含完全背包问题公式推导）](https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/)
    /// ` if (i - coin >= 0 && dp[i - coin] != amount + 1) {`，在找`dp[i]`时必须验证`dp[amount - coins[i]]`这个之前的是否存在，如果不存在说明无法使用凑出
    /// `dp[i]`。但是、但是、但是，对于从1..=amount开始dp[i]，dp[0]=0，dp[preidx=amount-coins[i]]是必存在的，
    /// 这个验证是不需要的，移除
    ///
    /// ```
    /// pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    ///     let amount = amount as usize;
    ///     let mut dp = vec![None; amount + 1];
    ///     // 理解 dp[0] = 0 的合理性，单独一枚硬币如果能够凑出面值，符合最优子结构
    ///     dp[0] = Some(0);
    ///     for amount in 1..=amount {
    ///         for coin in &coins {
    ///             let pre_idx = amount as i32 - coin;
    ///             if pre_idx >= 0 && dp[pre_idx as usize].is_some() {
    ///                 dp[amount] = dp[amount]
    ///                     // 避免 None.min(Some(v)) 返回一个None
    ///                     .or(Some(i32::MAX))
    ///                     .min(dp[pre_idx as usize].map(|v| v + 1))
    ///             }
    ///         }
    ///     }
    ///     dp[amount].unwrap_or(-1)
    /// }
    /// assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    /// ```
    ///
    /// 下面使用iter api替换内部for循环：
    ///
    /// ```
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
    /// assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    /// ```
    ///
    /// 参考:
    ///
    /// * [动态规划、完全背包、BFS（包含完全背包问题公式推导）](https://leetcode-cn.com/problems/coin-change/solution/dong-tai-gui-hua-shi-yong-wan-quan-bei-bao-wen-ti-/)
    /// * [[C++] O(n*amount) time O(amount) space DP solution](https://leetcode.com/problems/coin-change/discuss/77360/C%2B%2B-O(n*amount)-time-O(amount)-space-DP-solution)
    ///
    /// ### Submissions
    ///
    /// date=20210309, mem=2.1, mem_beats=50, runtime=16, runtime_beats=35, url=https://leetcode-cn.com/submissions/detail/152889099/
    ///
    /// date=20210310, mem=2.1, mem_beats=50, runtime=20, runtime_beats=35, url=https://leetcode-cn.com/submissions/detail/153326263/
    ///
    /// date=20210522, mem=2, mem_beats=86, runtime=16, runtime_beats=40, url=https://leetcode-cn.com/submissions/detail/179790819/
    ///
    /// date=20210523, mem=2, mem_beats=63, runtime=12, runtime_beats=60, url=https://leetcode-cn.com/submissions/detail/180025433/
    ///
    /// date=20210613, mem=2, mem_beats=83, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/186249323/
    ///
    /// date=20210716, mem=1.9, mem_beats=96, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/196350485/
    pub struct Solution;

    impl Solution {
        pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
            let (amount, mark) = (amount as usize, amount + 1);
            let mut dp = vec![mark; amount + 1];
            dp[0] = 0;
            for i in 1..=amount {
                for coin in &coins {
                    let coin = *coin as usize;
                    if i >= coin {
                        dp[i] = dp[i].min(dp[i - coin] + 1);
                    }
                }
            }
            if dp[amount] == mark {
                -1
            } else {
                dp[amount]
            }
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
    ///
    /// date=20210523, mem=2.1, mem_beats=46, runtime=12, runtime_beats=60, url=https://leetcode-cn.com/submissions/detail/180036176/
    ///
    /// date=20210524, mem=2.1, mem_beats=43, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180334942/
    ///
    /// date=20210613, mem=2, mem_beats=84, runtime=12, runtime_beats=64, url=https://leetcode-cn.com/submissions/detail/186258732/
    ///
    /// date=20210717, mem=2, mem_beats=80, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/196668718/
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

            let mut level = 0;
            while !queue.is_empty() {
                level += 1;
                for _ in 0..queue.len() {
                    let amount = queue.pop_front().unwrap();
                    for coin in &coins {
                        let rest_amount = amount - coin;

                        use std::cmp::Ordering::*;
                        match rest_amount.cmp(&0) {
                            Equal => return level,
                            Less => continue,
                            Greater => {
                                if !visited[rest_amount as usize] {
                                    visited[rest_amount as usize] = true;
                                    queue.push_back(rest_amount);
                                }
                            }
                        }
                    }
                }
            }
            // 未找出0 凑不出当前面值
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
