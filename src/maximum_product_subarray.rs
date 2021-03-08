//! 首次：
//! 
//! 0. 切分子数组找子数组的product
//! 0.0 切分：如果当前cur*pre_prod < cur
//!      pre_prod = cur.max(cur*pre_prod); max_prod = pre_prod.max(max_prod);
//! 1. 比较所有的products 找出最大的
//! 
//! 没有考虑到当`nums[i]<0`时对乘积的影响

pub mod solution_dp {
    /// # 思路
    ///
    /// `dp[i][j]`表示以 `nums[i]` 结尾的连续子数组的最值，计算最大值还是最小值由 j 来表示，j 就两个值；
    ///
    /// * 当 j = 0 的时候，表示计算的是最小值；
    /// * 当 j = 1 的时候，表示计算的是最大值。
    ///
    /// 由于之前的值与当前`nums[i]`的正负值会一起影响`dp[i][j]`：
    ///
    /// 当`nums[i] < 0`时最大值*`nums[i]`为最小值
    ///
    /// 要注意必须切分新的一段：
    ///
    /// 积`nums[i] * dp[i - 1].max|min`与当前`nums[i]`比较选择大，如`nums[i] > 0`时 `dp[i].max < 0`
    /// 表示切分i为新的一段开始
    ///
    /// ```
    /// dp[i].min = min(nums[i], nums[i] * dp[i - 1].min) if nums[i] >= 0
    /// dp[i].max = max(nums[i], nums[i] * dp[i - 1].max) if nums[i] >= 0
    ///
    /// dp[i].min = min(nums[i], nums[i] * dp[i - 1].max) if nums[i] < 0
    /// dp[i].max = max(nums[i], nums[i] * dp[i - 1].min) if nums[i] < 0
    /// ```
    ///
    /// 由于 nums[i] 必须被选取，那么 dp[i][0] = nums[0]，dp[i][1] = nums[0]。
    ///
    /// 参考：
    ///
    /// * [动态规划（理解无后效性）](https://leetcode-cn.com/problems/maximum-product-subarray/solution/dong-tai-gui-hua-li-jie-wu-hou-xiao-xing-by-liweiw/)
    ///
    /// ### Submissions
    ///
    /// date=20210308, mem=2.2, mem_beats=36, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152482019/
    pub struct Solution;

    impl Solution {
        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut dp = vec![(0, 0); nums.len()];
            dp[0] = (nums[0], nums[0]);
            let mut res = dp[0].1;
            for i in 1..nums.len() {
                let num = nums[i];
                dp[i] = if num >= 0 {
                    (num.min(num * dp[i - 1].0), num.max(num * dp[i - 1].1))
                } else {
                    (num.min(num * dp[i - 1].1), num.max(num * dp[i - 1].0))
                };
                res = res.max(dp[i].1);
            }
            res
        }
    }
}

pub mod solution_dp_optimized {
    /// 思路：
    ///
    /// 参考：
    ///
    /// * [画解算法：152. 乘积最大子序列](https://leetcode-cn.com/problems/maximum-product-subarray/solution/hua-jie-suan-fa-152-cheng-ji-zui-da-zi-xu-lie-by-g/)
    ///
    /// ### Submissions
    ///
    /// date=20210308, mem=2, mem_beats=81, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152464307/
    pub struct Solution;

    impl Solution {
        pub fn max_product(nums: Vec<i32>) -> i32 {
            let mut min_prod = nums[0];
            let mut max_prod = min_prod;
            let mut res = min_prod;
            for i in 1..nums.len() {
                if nums[i] < 0 {
                    std::mem::swap(&mut min_prod, &mut max_prod);
                }
                max_prod = nums[i].max(nums[i] * max_prod);
                min_prod = nums[i].min(nums[i] * min_prod);

                res = res.max(max_prod);
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::max_product);
        test(solution_dp_optimized::Solution::max_product);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![2, 3, -2, 4]), 6);
        assert_eq!(func(vec![-2, 0, -1]), 0);
        assert_eq!(func(vec![-2, 3, -4]), 24);
    }
}
