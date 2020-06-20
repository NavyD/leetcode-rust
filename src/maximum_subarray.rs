//! Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//! 
//! Example:
//! 
//! Input: [-2,1,-3,4,-1,2,1,-5,4],
//! Output: 6
//! Explanation: [4,-1,2,1] has the largest sum = 6.
//! Follow up:
//! 
//! If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

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

pub mod solution_divide_and_conquer {
    /// # 思路
    ///
    /// 用分治法将max sub array的sum用于两个数组上，如果能知道0..mid, mid+1..n两个数组对应
    /// 的max sub array：left_max_sum, right_max_sum. 就可以比较得到更大的sum，那如何找left, right的sum
    ///
    /// dp[0..n] = max(max(dp[0..=mid], dp[mid+1..n]), max_sum[0..n])
    ///
    /// 与归并中的递归相同，当left,right只有1个元素时，可以比较得到答案并回用于下个loop
    /// 
    /// 而对于max_sum(0, n)要保证最大的连续和，可用mid..left, mid..hi找最大和再加上就是最大和，利用mid to left找
    /// 连续和
    /// 
    /// ```ignore
    /// nums=[4, -1, 2, 1]
    /// mid=1, lo=0, hi=3
    /// left_max_sum = nums[1] + nums[0] = 3
    /// right_max_sum = nums[2] + nums[3] = 3
    /// 
    /// max_sum = left_max_sum + right_max_sum = 6
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200620, mem=2.1, mem_beats=92.56, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/355978516/
    /// 
    /// author=porker2008, references=https://leetcode.com/problems/maximum-subarray/discuss/20372/How-to-solve-"Maximum-Subarray"-by-using-the-divide-and-conquer-approach/20607
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N log N)
    /// 
    /// - 空间：O(log N) 存在log N的stack空间
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            if nums.is_empty() {
                0
            } else {
                Self::max_sum(&nums, 0, nums.len() - 1)
            }
        }

        fn max_sum(nums: &Vec<i32>, lo: usize, hi: usize) -> i32 {
            if lo >= hi {
                return nums[lo];
            }
            let mid = (lo + hi) / 2;
            // get max sub sum compare with left sum and right sum
            let sub_max_sum = Self::max_sum(nums, lo, mid)
                .max(Self::max_sum(nums, mid + 1, hi));
            // get current max sum with left and right
            let mut left_max_sum = nums[mid];
            let mut sum = 0;
            // get left sum from mid to lo
            for i in (lo..=mid).rev() {
                sum += nums[i];
                if sum > left_max_sum {
                    left_max_sum = sum;
                }
            }
            sum = 0;
            let mut right_max_sum = nums[mid + 1];
            // get right sum from mid+1 to hi
            for i in mid + 1..=hi {
                sum += nums[i];
                if sum > right_max_sum {
                    right_max_sum = sum;
                }
            }
            // to compare current sum and max sub sums
            (left_max_sum + right_max_sum).max(sub_max_sum)
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
