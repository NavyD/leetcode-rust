pub mod solution_dp {
    /// # 思路
    ///
    /// 正确的子问题定义应该是，`problem(k,i) = problem(k-1, i) + problem(k, i-k)`
    ///
    /// 即前k个硬币凑齐金额i的组合数等于前k-1个硬币凑齐金额i的组合数加上在原来i-k的基础上使用硬币的组合数。
    /// 说的更加直白一点，那就是用前k的硬币凑齐金额i，要分为两种情况考虑：
    ///
    /// - 一种是没有用前k-1个硬币就凑齐了，
    /// - 一种是前面已经凑到了i-k，现在就差第k个硬币了。
    ///
    /// 参考：
    ///
    /// * [Knapsack problem - Java solution with thinking process O(nm) Time and O(m) Space](https://leetcode.com/problems/coin-change-2/discuss/99212/Knapsack-problem-Java-solution-with-thinking-process-O(nm)-Time-and-O(m)-Space)
    /// * [零钱兑换II和爬楼梯问题到底有什么不同？](https://leetcode-cn.com/problems/coin-change-2/solution/ling-qian-dui-huan-iihe-pa-lou-ti-wen-ti-dao-di-yo/)
    ///
    /// ### Submissions
    ///
    /// date=20210310, mem=6.3, mem_beats=100, runtime=4, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153303521/
    pub struct Solution;

    impl Solution {
        pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
            let mut dp = vec![vec![0; amount as usize + 1]; coins.len() + 1];
            dp[0][0] = 1;
            for i in 1..=coins.len() {
                dp[i][0] = 1;
                for j in 1..=amount as usize {
                    dp[i][j] = dp[i - 1][j];
                    let pre_amount_idx = j as i32 - coins[i - 1];
                    if pre_amount_idx >= 0 {
                        dp[i][j] += dp[i][pre_amount_idx as usize];
                    }
                }
            }
            *dp.last().and_then(|a| a.last()).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::change);
    }

    fn test<F: Fn(i32, Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(5, vec![1, 2, 5]), 4);
        assert_eq!(func(3, vec![2]), 0);
        assert_eq!(func(10, vec![10]), 1);
    }
}
