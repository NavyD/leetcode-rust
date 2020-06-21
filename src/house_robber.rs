//! You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security system connected and it will automatically contact the police if two adjacent houses were broken into on the same night.
//!
//! Given a list of non-negative integers representing the amount of money of each house, determine the maximum amount of money you can rob tonight without alerting the police.
//!
//!  
//!
//! Example 1:
//!
//! Input: nums = [1,2,3,1]
//! Output: 4
//! Explanation: Rob house 1 (money = 1) and then rob house 3 (money = 3).
//!              Total amount you can rob = 1 + 3 = 4.
//! Example 2:
//!
//! Input: nums = [2,7,9,3,1]
//! Output: 12
//! Explanation: Rob house 1 (money = 2), rob house 3 (money = 9) and rob house 5 (money = 1).
//!              Total amount you can rob = 2 + 9 + 1 = 12.
//!  
//!
//! Constraints:
//!
//! 0 <= nums.length <= 100
//! 0 <= nums[i] <= 400

pub mod solution_dp {
    /// # 思路
    /// 
    /// 设dp[i]为可得到的最多的钱，由于不能连续取钱，
    /// 且i-1时可能是最多钱，可有`dp[i] = max(dp[i-1], dp[i - 2] + nums[i])`
    /// 
    /// 当dp[0]=nums[0]，dp[1]应该取max(nums[0], nums[1])
    /// 
    /// ```ignore
    /// nums=[2,4,1]
    /// dp[0]=2
    /// dp[1]=4 // max(nums[0], nums[1])
    /// dp[2]=4 // dp[2]=max(dp[1], dp[0]+nums[2])
    /// ```
    /// ## Submissions
    /// 
    /// date=20200621, mem=2.1, mem_beats=41.67, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356363887/
    /// 
    /// author=navyd
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 0 {
                return 0;
            } else if n == 1 {
                return nums[0];
            }
            let mut dp = vec![0; n];
            dp[0] = nums[0];
            dp[1] = nums[1].max(nums[0]);
            for i in 2..n {
                dp[i] = dp[i - 1].max(dp[i - 2] + nums[i]);
            }
            dp[n - 1]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let nums = vec![1, 2, 3, 1];
            assert_eq!(Solution::rob(nums), 4);

            let nums = vec![2, 1, 1, 2];
            assert_eq!(Solution::rob(nums), 4);
        }
    }
}

pub mod solution_dp_with_var {
    /// # 思路
    /// 
    /// 与[solution_dp](super::solution_dp::Solution)一样，只是不再需要数组，而是
    /// 用var保证中间状态
    /// 
    /// 考虑到`dp[i] = max(dp[i-1], dp[i - 2] + nums[i])`，只用到了dp[i-1],dp[i-2]，
    /// 可用2个变量替代
    /// 
    /// ## Submissions
    /// 
    /// date=20200621, mem=1.9, mem_beats=97.22, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/356376964/
    /// 
    /// author=heroes3001, references=https://leetcode.com/problems/house-robber/discuss/156523/From-good-to-great.-How-to-approach-most-of-DP-problems.
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            let n = nums.len();
            if n == 0 {
                return 0;
            }
            // is dp[i - 1]
            let mut prev1 = 0;
            // is dp[i - 2] + nums[i]
            let mut prev2 = 0;
            for num in nums {
                let t = prev1;
                prev1 = prev1.max(prev2 + num);
                prev2 = t;
            }
            prev1
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let nums = vec![1, 2, 3, 1];
            assert_eq!(Solution::rob(nums), 4);

            let nums = vec![2, 1, 1, 2];
            assert_eq!(Solution::rob(nums), 4);
        }
    }
}
