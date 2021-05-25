pub mod solution_dp {
    /// # 思路
    /// 
    /// 环状排列意味着第一个房子和最后一个房子中只能选择一个偷窃，
    /// 因此可以把此环状排列房间问题约化为两个单排排列房间子问题(198)：
    /// 
    /// - 在不偷窃第一个房子的情况下（即 nums[1:]），最大金额是p1；
    /// - 在不偷窃最后一个房子的情况下（即 nums[:n-1]），最大金额是p2。
    /// - 综合偷窃最大金额： 为以上两种情况的较大值，即 max(p1,p2)。
    /// 
    /// dp方程参考hourse_robber: `dp[i] = max(for 0..nums.len()-1: max(dp[i-1], dp[i - 2] + nums[i]), for 1..nums.len(): max(dp[i-1], dp[i - 2] + nums[i]))`
    /// 
    /// 参考：
    /// 
    /// * [打家劫舍 II（动态规划，结构化思路，清晰题解）](https://leetcode-cn.com/problems/house-robber-ii/solution/213-da-jia-jie-she-iidong-tai-gui-hua-jie-gou-hua-/)
    /// 
    /// ### Submissions
    ///
    /// date=20210525, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180620488/
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            fn helper(nums: &[i32]) -> i32 {
                let mut dp = vec![0; nums.len() + 1];
                dp[1] = nums[0];
                for i in 2..dp.len() {
                    dp[i] = dp[i - 1].max(dp[i - 2] + nums[i - 1]);
                }
                dp[nums.len()]
            }

            if nums.len() == 1 {
                nums[0]
            } else {
                helper(&nums[..nums.len() - 1]).max(helper(&nums[1..]))
            }
        }
    }
}

pub mod solution_dp_optimized {
    /// ### Submissions
    /// 
    /// date=20210525, mem=2, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180623818/
    pub struct Solution;

    impl Solution {
        pub fn rob(nums: Vec<i32>) -> i32 {
            fn helper(nums: &[i32]) -> i32 {
                let (mut pre, mut cur) = (0, 0);
                for num in nums {
                    let next = cur.max(pre + num);
                    pre = cur;
                    cur = next;
                }
                cur
            }

            if nums.len() == 1 {
                nums[0]
            } else {
                helper(&nums[..nums.len() - 1]).max(helper(&nums[1..]))
            }
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

    fn test<F: Fn(Vec<i32>) -> i32>(f: F) {
        assert_eq!(f(vec![2, 3, 2]), 3);
        assert_eq!(f(vec![1, 2, 3, 1]), 4);
        assert_eq!(f(vec![0]), 0);
        assert_eq!(f(vec![1]), 1);
    }
}
