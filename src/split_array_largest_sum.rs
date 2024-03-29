pub mod solution_dp {
    /// # 思路
    ///
    /// 考虑 所有前缀区间 分割成 2 个非空连续子数组的情况：
    ///
    /// - `[7]` 不能分割成 2 个非空连续子数组的和；
    /// - `[7, 2]` 分割成 2 个非空连续子数组，只有 1 种分割情况：`[7, | 2]` ，其中「`[7]` 分割成 1 个非空连续子数组」的情况我们在第 1 步计算过；
    /// - `[7, 2, 8]` 分割成 2 个非空连续子数组，有 2 种分割情况：
    /// - `[7, | 2, 8]` ，其中「`[7]` 分割成 1 个非空连续子数组」的情况我们在第 1 步计算过；
    /// - `[7, 2, | 8]` ，其中「`[7, 2]` 分割成 1 个非空连续子数组」的情况我们在第 2 步计算过；
    ///
    /// ![](https://pic.leetcode-cn.com/1605296634-OrhsWh-image.png)
    ///
    /// `dp[i][k]` 表示：将前缀区间 `[0, i]` 被分成 k 段的各自和的最大值的最小值记为 `dp[i][k]`，
    /// 那么前缀区间 `[0, j]` （这里 j < i） 被分成 k - 1 段各自和的最大值的最小值为 `dp[j][k - 1]`
    ///
    /// dp方程：`dp[i][k]=max(dp[j][k−1],rangeSum(j+1,i))`
    ///
    /// ```ignore
    /// for k in 2..=m {
    ///     for i in k - 1..n {
    ///         for j in k - 2..i {
    ///             dp[i][k] = dp[i][k].min(dp[j][k-1].max(presums[i] - presums[j+1]))
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ![](https://pic.leetcode-cn.com/1605299059-MXsGDC-image.png)
    ///
    /// 初始化：由于要找最小值，初值赋值成为一个不可能达到的很大的值；分割数为 1 ，即不分割的情况，所有的前缀和就是依次的状态值。
    ///
    /// 参考：
    ///
    /// - [动态规划、二分查找（Java）](https://leetcode-cn.com/problems/split-array-largest-sum/solution/er-fen-cha-zhao-by-liweiwei1419-4/)
    ///
    /// ### Submissions
    ///
    /// date=20210917, mem=2.1, mem_beats=100, runtime=80, runtime_beats=20
    ///
    /// date=20210918, mem=2.2, mem_beats=50, runtime=160, runtime_beats=100
    ///
    /// date=20211012, mem=2.3, mem_beats=16, runtime=80, runtime_beats=33
    pub struct Solution;

    impl Solution {
        pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
            let (n, m) = (nums.len(), m as usize);

            let mut presums = vec![0; n + 1];
            // 前缀和
            (0..n).for_each(|i| presums[i + 1] = presums[i] + nums[i]);

            let mut dp = vec![vec![i32::MAX; m + 1]; n];
            // 分割数为 1 ，即不分割的情况，所有的前缀和就是依次的状态值
            (0..n).for_each(|i| dp[i][1] = presums[i + 1]);

            for k in 2..=m {
                for i in k - 1..n {
                    for j in k - 2..i {
                        dp[i][k] = dp[i][k].min(
                            // max sum of subarray
                            dp[j][k - 1].max(presums[i + 1] - presums[j + 1]),
                        );
                    }
                }
            }
            dp[n - 1][m]
        }
    }
}

pub mod solution_binary {
    /// # 思路
    ///
    /// 挖掘单调性：使用二分查找的一个前提是「数组具有单调性」，我们就去想想有没有单调性可以挖掘，不难发现：
    ///
    /// - 如果设置「数组各自和的最大值」很大，那么必然导致分割数很小；
    /// - 如果设置「数组各自和的最大值」很小，那么必然导致分割数很大。
    ///
    /// 我们就可以通过调整「数组各自和的最大值」来达到：使得分割数恰好为 m 的效果。
    ///
    /// 这里要注意一个问题：
    ///
    /// 如果某个 数组各自和的最大值 恰恰好使得分割数为 m ，此时不能放弃搜索，因为我们要使得这个最大值 最小化，
    /// 此时还应该继续尝试缩小这个 数组各自和的最大值 ，使得分割数超过 m ，
    /// 超过 m 的最后一个使得分割数为 m 的 数组各自和的最大值 就是我们要找的 最小值
    ///
    /// 参考：
    ///
    /// - [动态规划、二分查找（Java）](https://leetcode-cn.com/problems/split-array-largest-sum/solution/er-fen-cha-zhao-by-liweiwei1419-4/)
    ///
    /// ### Submissions
    ///
    /// date=20210917, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100
    ///
    /// date=20210918, mem=2, mem_beats=100, runtime=0, runtime_beats=100
    ///
    /// date=20211012, mem=2, mem_beats=66, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
            /// 计算满足不超过「子数组各自的和的最大值sum」的分割数
            fn splits(nums: &[i32], sum: i32) -> i32 {
                let mut count = 1;
                let mut sub_arr_sum = 0;
                for num in nums {
                    sub_arr_sum += num;
                    if sub_arr_sum > sum {
                        count += 1;
                        sub_arr_sum = *num;
                    }
                }
                count
            }

            // 子数组各自的和的最大值的上下界
            let (mut left_sum, mut right_sum) = nums
                .iter()
                .fold((0, 0), |(max_num, sum), num| (max_num.max(*num), sum + num));

            while left_sum < right_sum {
                let mid_sum = (left_sum + right_sum) / 2;
                // 如果分割数太多 增大sum 减小分割数
                if splits(&nums, mid_sum) > m {
                    left_sum = mid_sum + 1;
                } else {
                    // 分割数为 m时继续搜索
                    right_sum = mid_sum;
                }
            }
            left_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::split_array);
        test(solution_binary::Solution::split_array);
    }

    fn test<F: Fn(Vec<i32>, i32) -> i32>(f: F) {
        assert_eq!(f(vec![7, 2, 5, 10, 8], 2), 18);
        assert_eq!(f(vec![1, 2, 3, 4, 5], 2), 9);
        assert_eq!(f(vec![1, 4, 4], 3), 4);
    }
}
