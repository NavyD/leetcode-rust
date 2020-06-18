pub mod solution_dp {
    /// # 思路
    ///
    /// 假定dp[n]表示0-n的max subarray的和，对dp[i]有：
    /// 如果dp[i-1]<=0，则dp[i] = nums[i]，
    /// 如果dp[i-1]>0，则dp[i] = nums[i] + dp[i-1]，
    /// 即：`dp[i] = nums[i] + max(dp[i-1], 0)`
    ///
    /// ```ignore
    /// nums=[-2,-1]
    /// dp[0] = nums[0] = -2
    /// dp[1] = nums[1] + dp[0] > 0 ? dp[0] : 0 = -1
    /// ```
    ///
    /// ## 问题
    ///
    /// - 不能用nums[i]>0保证nums[i]>0，则`dp[i]=dp[i-1]+nums[i]
    ///
    /// 如果nums[i]<=0，则保持`dp[i]=dp[i-1]`，
    /// 如果nums[i]>0，则`dp[i]=dp[i-1]+nums[i]`，
    /// 即有：`dp[i] = max(dp[i-1], dp[i-1] + nums[i])`。设dp[0]=0，
    /// dp[1] = d，这段分析是错误的：
    ///
    /// 由于max subarray是求和，当nums[i]<=0时有dp[i]=dp[i-1]，
    /// 那dp[i-1] < nums[i]就不成立了：nums[1]=-1, dp[0]=-2
    ///
    /// ## submissions
    ///
    /// date=20200618, mem=2.2, mem_beats=58.91, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/355190523/
    ///
    /// author=FujiwaranoSai, references=https://leetcode.com/problems/maximum-subarray/discuss/20193/DP-solution-and-some-thoughts
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    ///
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            if nums.is_empty() {
                return 0;
            }
            // create dp
            let mut dp = vec![0; nums.len()];
            dp[0] = nums[0];
            let mut largest: i32 = dp[0];
            for i in 1..nums.len() {
                // dp[i] = nums[i] + max(dp[i-1], 0)
                dp[i] = nums[i] + 0.max(dp[i - 1]);
                largest = largest.max(dp[i]);
            }
            largest
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
            assert_eq!(Solution::max_sub_array(nums), 6);

            let nums = vec![-2, 1, -3];
            assert_eq!(Solution::max_sub_array(nums), 1);

            let nums = vec![-2];
            assert_eq!(Solution::max_sub_array(nums), -2);

            let nums = vec![];
            assert_eq!(Solution::max_sub_array(nums), 0);

            let nums = vec![-2, -1];
            assert_eq!(Solution::max_sub_array(nums), -1);
        }
    }
}

#[allow(dead_code)]
mod solution_max_sum {
    /// - 如果dp[n]定义为前n位的和而不是最大的和
    ///
    /// dp[n]表示在n位nums的和，dp[0]=0, dp[1] = dp[0] + nums[0]
    /// 找max sum的max_index为max_index dp[i-1]，
    /// 在max_index中找最小的sum min_index，
    /// sum = dp[max_index] - dp[min_index]，
    /// 这里尝试使用index的方式取中间一段为max subarray，
    /// 但是无法解决
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            if nums.len() == 0 {
                return 0;
            }
            if nums.len() == 1 {
                return nums[0];
            }
            let mut sums = vec![0; nums.len() + 1];
            let (mut max_idx, mut min_idx) = (0, 0);
            let (mut max_sum, mut min_sum) = (nums[max_idx], nums[min_idx]);
            for i in 0..nums.len() {
                let sum = nums[i] + sums[i];
                // 0. find max sum and index
                if sum > max_sum {
                    max_idx = i;
                    max_sum = sum;
                }
                sums[i + 1] = sum;
            }

            for i in 0..max_idx {
                // 1. find min sum and index in max index
                let sum = sums[i + 1];
                if sum < min_sum {
                    min_idx = i;
                    min_sum = sum;
                }
            }
            // sum in min and max
            sums[max_idx + 1] - sums[min_idx + 1]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
            assert_eq!(Solution::max_sub_array(nums), 6);

            let nums = vec![-2, 1, -3];
            assert_eq!(Solution::max_sub_array(nums), 1);

            let nums = vec![-2];
            assert_eq!(Solution::max_sub_array(nums), -2);

            let nums = vec![];
            assert_eq!(Solution::max_sub_array(nums), 0);
            let nums = vec![-2, -1];
            assert_eq!(Solution::max_sub_array(nums), -1);
        }
    }
}
