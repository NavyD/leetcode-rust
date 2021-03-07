/// 首次的想法：dp[i] = dp[i-1].max(dp[i - 1] + nums[i])
///
/// 没有考虑到：当nums[i]<0时必定有dp[i] < dp[i-1]+nums[i]，但此时一个
/// 最大子序列不一定到i完结了，这个递推公式不可行
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
pub mod solution_dp {
    /// # 思路
    ///
    /// 定义dp[i]表示在当前i元素时为结尾时的子序列的和，只要找到每个位置的子序列和比较就可以
    /// 找出最大值子序列和。考虑nums[i]是单独开始一段还是加入上一段，单nums[i]<0并不能决定，可以
    /// 比较：`dp[i] = nums[i].max(dp[i - 1] + nums[i])`
    ///
    /// 参考：
    ///
    /// * [最大子序和](https://leetcode-cn.com/problems/maximum-subarray/solution/zui-da-zi-xu-he-by-leetcode-solution/)
    /// * [最大子序和 c++实现四种解法 暴力法、动态规划、贪心法和分治法 图示讲解](https://leetcode-cn.com/problems/maximum-subarray/solution/zui-da-zi-xu-he-cshi-xian-si-chong-jie-fa-bao-li-f/)
    ///
    /// ## submissions
    ///
    /// date=20200618, mem=2.2, mem_beats=58.91, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/355190523/
    ///
    /// date=20210222, mem=2, mem_beats=93, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/147575169/
    ///
    /// date=20210307, mem=2, mem_beats=87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/152088296/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    ///
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let mut sub_sum = nums[0];
            let mut longest = sub_sum;
            for i in 1..nums.len() {
                sub_sum = (sub_sum + nums[i]).max(nums[i]);
                longest = longest.max(sub_sum);
            }
            longest
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
            let sub_max_sum = Self::max_sum(nums, lo, mid).max(Self::max_sum(nums, mid + 1, hi));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::max_sub_array);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(func(vec![-2, 1, -3]), 1);
        assert_eq!(func(vec![-2]), -2);
        assert_eq!(func(vec![-2, -1]), -1);
    }
}
