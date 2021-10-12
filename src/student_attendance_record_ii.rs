pub mod solution_backtracking {
    ///  has overflowed its stack
    pub struct Solution;

    impl Solution {
        /// # 思路
        ///
        /// 相当于有 n 个位置，每个位置可以放 P/A/L 三个字符中的一个，但是有一些限制，总共的 A 不能超过 2 个，不能有连续3个的 L。
        /// 在一天的出勤中3种情况只可能同时出现一种，因此在枚举/统计合法方案的个数时，
        /// 当我们决策到某一位应该选什么时，我们关心的是当前方案中已经出现了多少个 A（以决策当前能否填入 A）以及连续出现的 L 的次数是多少（以决策当前能否填入 L）。
        ///
        /// 在递归中参数present不用考虑传入，只考虑其它2种的影响。
        /// 缓存(day,absents,lates) 的方案数以避免重复计算
        ///
        /// 参考：
        ///
        /// - [【宫水三叶】一题三解 :「记忆化搜索」&「动态规划」&「矩阵快速幂」](https://leetcode-cn.com/problems/student-attendance-record-ii/solution/gong-shui-san-xie-yi-ti-san-jie-ji-yi-hu-fdfx/)
        /// - [【彤哥来刷题啦】一题六解：DFS -> 记忆化 -> DP -> 降维 -> 降维 -> 滚动数组，娓娓道来，小白都能看懂](https://leetcode-cn.com/problems/student-attendance-record-ii/solution/tong-ge-lai-shua-ti-la-yi-ti-liu-jie-dfs-s5fa/)
        ///
        /// ### Submissions
        ///
        /// date=20211012, mem=24.3, mem_beats=8, runtime=380, runtime_beats=5
        pub fn check_record(n: i32) -> i32 {
            const MOD: i32 = 1_000_000_007;
            const MARK: i32 = -1;

            fn backtrack(
                day: usize,
                absents: usize,
                lates: usize,
                cached_rewards: &mut [Vec<Vec<i32>>],
            ) -> i32 {
                if absents >= 2 || lates >= 3 {
                    return 0;
                }
                if day == 0 {
                    return 1;
                }
                if cached_rewards[day][absents][lates] != MARK {
                    return cached_rewards[day][absents][lates];
                }

                let next_day = day - 1;
                // absent
                let mut reward = backtrack(next_day, absents + 1, 0, cached_rewards) % MOD;
                // late
                reward = (reward + backtrack(next_day, absents, lates + 1, cached_rewards)) % MOD;
                // present
                reward = (reward + backtrack(next_day, absents, 0, cached_rewards)) % MOD;

                cached_rewards[day][absents][lates] = reward;
                reward
            }
            let n = n as usize;
            backtrack(n, 0, 0, &mut vec![vec![vec![MARK; 3]; 2]; n + 1])
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 状态定义：dp[i][j][k]表示第 i 天、在 A 为 j 次、连续的 L 为 k 次的方案数。
    /// 状态转移：所有的状态都是从前一天，即 i-1，转移而来，但是对于 j 和 k，要分三种情况来讨论：
    ///
    /// - 当前填入的是 P，i-1 天的任何状态都能转移过来：昨天是2种A[0,1],每种对应3种L
    /// - 当前填入的是 A，i-1 天即之前肯定没填过 A，同时所有的 late 状态都可以转移到过来：昨天是P 或对应3种L
    /// - 当前填入的是 L，i-1 天最多只能有一个连续的 L，其他的状态依次转移过来：
    ///
    /// ### Submissions
    ///
    /// date=20211012, mem=16.3, mem_beats=31, runtime=244, runtime_beats=21
    pub struct Solution;

    impl Solution {
        pub fn check_record(n: i32) -> i32 {
            const MOD: i64 = 1_000_000_007;

            let n = n as usize;
            let mut dp = vec![vec![vec![0; 3]; 2]; n];
            dp[0][0][0] = 1;
            dp[0][1][0] = 1;
            dp[0][0][1] = 1;

            for i in 1..n {
                // present
                dp[i][0][0] = dp[i - 1][0].iter().sum::<i64>() % MOD;
                dp[i][1][0] = dp[i - 1][1].iter().sum::<i64>() % MOD;

                // absent
                dp[i][1][0] = (dp[i][1][0] + dp[i - 1][0].iter().sum::<i64>()) % MOD;

                // late
                dp[i][0][1] = dp[i - 1][0][0];
                dp[i][0][2] = dp[i - 1][0][1];
                dp[i][1][1] = dp[i - 1][1][0];
                dp[i][1][2] = dp[i - 1][1][1];
            }
            // 最后一天的所有状态相加
            dp[n - 1]
                .iter()
                .flatten()
                .fold(0, |res, acc| (res + acc) % MOD) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // fixed stack error: over flow
        std::thread::Builder::new()
            .stack_size(5 * (1 << 20)) // 5MB
            .spawn(|| test(solution_backtracking::Solution::check_record))
            .unwrap()
            .join()
            .unwrap();
        test(solution_dp::Solution::check_record);
    }

    fn test<F: Fn(i32) -> i32>(f: F) {
        assert_eq!(f(2), 8);
        assert_eq!(f(1), 3);
        assert_eq!(f(10), 3536);
        assert_eq!(f(20), 2947811);
        assert_eq!(f(10101), 183236316);
    }
}
