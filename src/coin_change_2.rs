/// 思考：为何在coin change中的bfs解法在这里不可行
///
/// 简单的修改`if rest_amount==0 { res += 1 }`时，状态树由于visited的存在，会将
/// 多个分支到同一个rest_amount后合并为一种，使得结果少于组合数
///
/// 与coin change不同，coin change可以同时使用permutations&combinations，交换for coin,for amount
/// 内外循环是不影响结果的
pub mod solution_dp {
    /// # 思路
    ///
    /// 在coin change中是要找出花费最少个数的硬币，如果在这里改成计算硬币数的和：
    /// `dp[amount] = for coin in coins: if coin <= amount: sum(dp[amount - coins[i]]))`。
    /// 但是结果不对，这里找出的是硬币的排列数，`1,2`,`2,1`被视为两种情况，而题目要求的是组合数，
    ///
    /// 正确的子问题定义应该是，`problem(k,i) = problem(k-1, i) + problem(k, i-k)`
    ///
    /// 即前k个硬币凑齐金额i的组合数等于前k-1个硬币凑齐金额i的组合数加上在原来i-k的基础上使用硬币的组合数。
    /// 说的更加直白一点，那就是用前k的硬币凑齐金额i，要分为两种情况考虑：
    ///
    /// - 一种是用k-1个硬币就凑齐了：`dp[k - 1][i]`
    /// - 一种是前面已经凑到了i-k，现在就差第k个硬币了：`dp[k][i-k]`
    ///
    /// dp方程：`dp[k][i] = dp[k][i - coin] + dp[k - 1][i] if coin <= i else dp[k - 1][i] for coin in coins`
    ///
    /// 初始化：使`dp.len=coins.len + 1, dp[i].len=amount+1`，当只有1个硬币且刚好amount=coin
    /// 即`k=1,i=1`时`dp[1][1]=dp[0][1] + dp[1, 0]=1`，此时应该初始化dp[i][0]=1
    ///
    /// 参考：
    ///
    /// * [Knapsack problem - Java solution with thinking process O(nm) Time and O(m) Space](https://leetcode.com/problems/coin-change-2/discuss/99212/Knapsack-problem-Java-solution-with-thinking-process-O(nm)-Time-and-O(m)-Space)
    /// * [零钱兑换II和爬楼梯问题到底有什么不同？](https://leetcode-cn.com/problems/coin-change-2/solution/ling-qian-dui-huan-iihe-pa-lou-ti-wen-ti-dao-di-yo/)
    ///
    /// ### Submissions
    ///
    /// date=20210310, mem=6.3, mem_beats=100, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153303521/
    ///
    /// date=20210311, mem=6.2, mem_beats=100, runtime=12, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153786239/
    ///
    /// date=20210524, mem=6.3, mem_beats=6.25, runtime=12, runtime_beats=6.25, url=https://leetcode-cn.com/submissions/detail/180329906/
    ///
    /// date=20210525, mem=6.3, mem_beats=6.25, runtime=12, runtime_beats=6.25, url=https://leetcode-cn.com/submissions/detail/180633984/
    ///
    /// date=20210613, mem=6.2, mem_beats=16, runtime=8, runtime_beats=24, url=https://leetcode-cn.com/submissions/detail/186278937/
    ///
    /// 将`dp[i][0] = 1`放入`for i in 1..=coins_len {`导致执行用时太慢，单独使用for_each使用时降到0ms
    /// date=20210717, mem=6.2, mem_beats=18, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/196659784/
    ///
    /// date=20210809, mem=6.3, mem_beats=13, runtime=8, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204905745/
    pub struct Solution;

    impl Solution {
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let (amount, coins_len) = (amount as usize, coins.len());
            let mut dp = vec![vec![0; amount + 1]; coins_len + 1];
            // 初始化
            (0..=coins_len).for_each(|i| dp[i][0] = 1);

            for i in 1..=coins_len {
                let coin = coins[i - 1] as usize;
                for j in 1..=amount {
                    dp[i][j] = dp[i - 1][j];
                    if j >= coin {
                        dp[i][j] += dp[i][j - coin];
                    }
                }
            }
            dp[coins_len][amount]
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// `dp[coin][amount]`中只使用`coin-1`优化：对于硬币从0到k，我们必须使用第k个硬币的时候，当前金额的组合数。
    /// 因此状态数组DP[i]表示的是对于第k个硬币能凑的组合数
    ///
    /// 状态转移方程如下`DP[i] = DP[i] + DP[i-k] for k in coins`
    ///
    /// 参考：
    ///
    /// * [Knapsack problem - Java solution with thinking process O(nm) Time and O(m) Space](https://leetcode.com/problems/coin-change-2/discuss/99212/Knapsack-problem-Java-solution-with-thinking-process-O(nm)-Time-and-O(m)-Space)
    /// * [零钱兑换II和爬楼梯问题到底有什么不同？](https://leetcode-cn.com/problems/coin-change-2/solution/ling-qian-dui-huan-iihe-pa-lou-ti-wen-ti-dao-di-yo/)
    ///
    /// ### Submissions
    ///
    /// date=20210311, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153796187/
    ///
    /// date=20210314, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/155003732/
    ///
    /// date=20210524, mem=1.9, mem_beats=100, runtime=0, runtime_beats=81, url=https://leetcode-cn.com/submissions/detail/180333918/
    ///
    /// date=20210524, mem=2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180634859/
    ///
    /// date=20210613, mem=2.1, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/186281437/
    ///
    /// date=20210717, mem=2.1, mem_beats=52, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/196663774/
    ///
    /// date=20210809, mem=2, mem_beats=68, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204928097/
    pub struct Solution;

    impl Solution {
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let amount = amount as usize;
            let mut dp = vec![0; amount + 1];
            dp[0] = 1;
            for coin in coins {
                let coin = coin as usize;
                for amount in coin..=amount {
                    dp[amount] += dp[amount - coin];
                }
            }
            dp[amount]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::change);
        test(solution_dp_optimized::Solution::change);
    }

    fn test<F: Fn(i32, Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(5, vec![1, 2, 5]), 4);
        assert_eq!(func(3, vec![2]), 0);
        assert_eq!(func(10, vec![10]), 1);
    }
}
