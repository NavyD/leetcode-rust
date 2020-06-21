//! You are climbing a stair case. It takes n steps to reach to the top.
//! 
//! Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//! 
//! Note: Given n will be a positive integer.
//! 
//! Example 1:
//! 
//! Input: 2
//! Output: 2
//! Explanation: There are two ways to climb to the top.
//! 1. 1 step + 1 step
//! 2. 2 steps
//! Example 2:
//! 
//! Input: 3
//! Output: 3
//! Explanation: There are three ways to climb to the top.
//! 1. 1 step + 1 step + 1 step
//! 2. 1 step + 2 steps
//! 3. 2 steps + 1 step

pub mod solution_dp {

    /// # 思路
    /// 
    /// 设dp[i]为在i个梯子时可走的步数，第i个梯子可以是i-1和i-2个上来的，
    /// 可有`dp[i] = dp[i-1] + dp[i-2]`
    /// 
    /// ```ignore
    /// dp[0] = 1
    /// dp[1] = 1
    /// dp[2] = 2
    /// dp[3] = 3
    /// dp[4] = 5 // 1111, 112, 121, 211, 22
    /// dp[5] = 8 // 11111, 1112, 1121, 1211, 2111, 122, 212, 221
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200621, mem=2, mem_beats=55.93, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356338066/,
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn climb_stairs(n: i32) -> i32 {
            let n = n as usize;
            let mut dp = vec![1; n + 1];
            for i in 2..=n {
                dp[i] = dp[i - 1] + dp[i - 2];
            }
            dp[n]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(Solution::climb_stairs(0), 1);
            assert_eq!(Solution::climb_stairs(1), 1);
            assert_eq!(Solution::climb_stairs(2), 2);
            assert_eq!(Solution::climb_stairs(3), 3);
            assert_eq!(Solution::climb_stairs(4), 5);
        }
    }
}
