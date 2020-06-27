//! On a staircase, the i-th step has some non-negative cost cost[i] assigned (0 indexed).
//!
//! Once you pay the cost, you can either climb one or two steps. You need to find minimum cost to reach the top of the floor, and you can either start from the step with index 0, or the step with index 1.
//!
//! Example 1:
//! Input: cost = [10, 15, 20]
//! Output: 15
//! Explanation: Cheapest is start on cost[1], pay that cost and go to the top.
//! Example 2:
//! Input: cost = [1, 100, 1, 1, 1, 100, 1, 1, 100, 1]
//! Output: 6
//! Explanation: Cheapest is start on cost[0], and only step on 1s, skipping cost[3].
//! Note:
//! cost will have a length in the range [2, 1000].
//! Every cost[i] will be an integer in the range [0, 999].

pub mod solution_dp {
    /// # 思路
    /// 
    /// 设dp[n]为爬到n个梯子的最小成本 `dp[i] = min(dp[i-1] , dp[i-2]) + cost[i])`，
    /// 第i个梯子可由i-1, i-2梯子上来，同时算上当前梯子的成本。
    ///
    /// 注意在n=cost.len()时是没有cost的，应该用`dp[n]=min(dp[n-1], dp[n-2])`，最后一步是没有cost的，
    /// 这一点非常容易混淆，可能导致`dp[i]=min(dp[i-1], dp[i-2] + cost[i])`或`dp[n=cost.len]=dp[n-1]`，
    /// 或`dp[0]=cost[0], dp[1]=min(cost[0], cost[1]`，
    /// [一个doubt解释](https://leetcode.com/problems/min-cost-climbing-stairs/discuss/148907/Doubt)
    /// 
    /// 
    ///
    /// 不能简单的dp[i] = min(dp[i-1], dp[i-2])必须考虑到dp[i]是否由间隔上来的i = i-3的可能，
    /// 在初始化时保证i不可能为i-3: `dp[1] = cost[1], dp[n=2] = min(dp[0], dp[1])`
    /// 
    /// ```ignore
    /// dp[0] = cost[0]
    /// dp[1] = cost[1]
    /// dp[2] = cost[2] + min(dp[0], dp[1])
    ///
    /// dp[2=n=cost.len()] = min(dp[1], dp[0])
    ///
    /// [10, 15, 20, 10, 5, 20]
    /// dp[0] = 10 = cost[0]
    /// dp[1] = 15 = cost[1]
    /// dp[2] = 30 = min(dp[1], dp[0]) + cost[2]
    /// dp[3] = 25 = min(dp[2], dp[1]) + cost[3]
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200627, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/358800877/,
    /// 
    /// author=claytonjwong, references=https://leetcode.com/problems/min-cost-climbing-stairs/discuss/110111/Javascript-and-C%2B%2B-solutions
    /// 
    /// author=mankadjyot, references=https://leetcode.com/problems/min-cost-climbing-stairs/discuss/148907/Doubt
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
            // let n = cost.len();
            // let mut dp = vec![0; n];
            // dp[0] = cost[0];
            // dp[1] = cost[1];
            // for i in 2..n {
            //     dp[i] = dp[i - 1].min(dp[i - 2]) + cost[i];
            // }
            // dp[n - 1].min(dp[n - 2])
            let n = cost.len();
            // more close
            let mut prev0 = cost[1];
            // less close
            let mut prev1 = cost[0];
            for i in 2..n {
                let t = prev0;
                prev0 = prev0.min(prev1) + cost[i];
                prev1 = t;
            }
            prev0.min(prev1)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let cost = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
            assert_eq!(Solution::min_cost_climbing_stairs(cost), 6);

            let cost = vec![10, 15, 20];
            assert_eq!(Solution::min_cost_climbing_stairs(cost), 15);
        }
    }
}
