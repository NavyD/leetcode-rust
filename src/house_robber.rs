pub mod solution_dp {
    /// # 思路
    ///
    /// 子问题：定义在下标为i的的最高金额：`problem(i) = max(problem(i - 1), problem(i - 2) + nums[i])`
    ///
    /// dp状态：设dp[i]为可得到的最多的钱
    ///
    /// dp方程：由于不能连续取钱，且i-1时可能是最多钱，可有`dp[i] = max(dp[i-1], dp[i - 2] + nums[i])`
    ///
    /// 初始化：当dp[0]=nums[0]，dp[1]应该取max(nums[0], nums[1])
    ///
    /// ```ignore
    /// nums=[2,4,1]
    /// dp[0]=2
    /// dp[1]=4 // max(nums[0], nums[1])
    /// dp[2]=4 // dp[2]=max(dp[1], dp[0]+nums[2])
    /// ```
    ///
   /// 注意：如果`dp.len() = nums.len() + 1`，使用`dp[0] = 0, dp[1]=nums[0]`，
    /// 这样就不需要判断了：
    ///
    /// ```ignore
    /// pub fn rob(nums: Vec<i32>) -> i32 {
    ///     if nums.is_empty() {
    ///         return 0;
    ///     }
    ///     let mut dp = vec![0; nums.len() + 1];
    ///     dp[1] = nums[0];
    ///     for i in 2..dp.len() {
    ///         dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1]);
    ///     }
    ///     *dp.last().unwrap()
    /// }
    /// ```
    /// 
    /// 参考：
    /// 
    /// * [图解动态规划的解题四步骤（C++/Java/Python）](https://leetcode-cn.com/problems/house-robber/solution/dong-tai-gui-hua-jie-ti-si-bu-zou-xiang-jie-cjavap/)
    ///
    /// ## Submissions
    ///
    /// date=20200621, mem=2.1, mem_beats=41.67, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356363887/
    ///
    /// date=20210314, mem=2, mem_beats=65, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/154988374/
    /// 
    /// date=20210517, mem=2.1, mem_beats=11, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/178198257/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let mut dp = nums;
            if dp.len() > 1 {
                dp[1] = dp[0].max(dp[1]);
                for i in 2..dp.len() {
                    dp[i] = dp[i - 1].max(dp[i - 2] + dp[i]);
                }
            }
            *dp.last().unwrap()
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 与[solution_dp](super::solution_dp::Solution)一样，只是不再需要数组，而是
    /// 用var保证中间状态
    ///
    /// 考虑到`dp[i] = max(dp[i-1], dp[i - 2] + nums[i])`，只用到了dp[i-1],dp[i-2]，
    /// 可用2个变量替代
    ///
    /// 参考：
    ///
    /// * [From good to great. How to approach most of DP problems.](https://leetcode.com/problems/house-robber/discuss/156523/From-good-to-great.-How-to-approach-most-of-DP-problems)
    ///
    /// ## Submissions
    ///
    /// date=20200621, mem=1.9, mem_beats=97.22, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356376964/
    ///
    /// date=20210517, mem=2, mem_beats=40, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/178201670/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            // is dp[i - 2] + nums[i]
            let mut pre = 0;
            // is dp[i - 1]
            let mut cur = 0;
            for num in nums {
                let next = cur.max(pre + num);
                pre = cur;
                cur = next;
            }
            cur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::rob);
        test(solution_dp_optimized::Solution::rob);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![1, 2, 3, 1]), 4);
        assert_eq!(func(vec![2, 1, 1, 2]), 4);
        assert_eq!(func(vec![0]), 0);
    }
}
