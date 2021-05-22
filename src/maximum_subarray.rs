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
    /// 子问题：定义problem(i)表示以下标为i为结尾的子序和。一个负数 + 任何数都会比当前数小，
    /// 则有：`problem(i) = problem(i - 1).max(0) + nums[i]`
    ///
    /// 状态数组：`dp[i]`表示以下标为i为结尾的子序和，最大子序和可以通过比较所有的dp[i]找出最大
    ///
    /// dp方程：考虑nums[i]是单独开始一段还是加入上一段，单`nums[i]<0`并不能决定：
    /// `dp[i] = max(nums[i], nums[i] + dp[i - 1])`
    ///
    /// 初始化：`dp[0] = nums[0]`
    ///
    /// ### Submissions
    ///
    /// date=20210314, mem=2.3, mem_beats=5, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/154951229/
    ///
    /// date=20210522, mem=2, mem_beats=79, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179758836/
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let (mut res, mut dp) = (nums[0], nums);
            for i in 1..dp.len() {
                dp[i] = dp[i].max(dp[i] + dp[i - 1]);
                res = res.max(dp[i]);
            }
            res
        }
    }
}

pub mod solution_dp_optimized {
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
    /// date=20210314, mem=2.1, mem_beats=41, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/154952557/
    ///
    /// date=20210522, mem=2, mem_beats=98, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179759906/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    ///
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            let (mut res, mut sub_sum) = (nums[0], nums[0]);
            for i in 1..nums.len() {
                sub_sum = nums[i].max(nums[i] + sub_sum);
                res = res.max(sub_sum);
            }
            res
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
    /// 注意：在取整个[lo..hi]跨中心范围和时，在left找最大值要从mid->lo反向找，由于
    /// 在使用最大值的方式从lo -> mid时当`mid<0`时无法将nums[mid]加入最大和中出现错误。
    /// 如：`[2,3,-6,2,4]` 当mid=2,nums[mid]=-6时left=[2,3,-6]，如果使用`lo->mid`则
    /// 会有left_max_sum = 5，而没有算入-6的结果left_max_sum = -1
    /// 
    /// 对于寻找跨中心范围可使用iter::fold api，由于要缓存中间结果，写起来也不是很优雅：
    /// 
    /// ```ignore
    /// let mut sum = 0;
    /// let left_max_sum1 = (lo..=mid).rfold(nums[mid], |acc, i| {
    ///     sum += nums[i];
    ///     sum.max(acc)
    /// });
    /// let mut sum = 0;
    /// let right_max_sum1 = (mid + 1..=hi).fold(nums[mid + 1], |acc, i| {
    ///     sum += nums[i];
    ///     sum.max(acc)
    /// });
    /// ```
    ///
    /// 参考：
    /// 
    /// * [How-to-solve-"Maximum-Subarray"-by-using-the-divide-and-conquer-approach](https://leetcode.com/problems/maximum-subarray/discuss/20372/How-to-solve-"Maximum-Subarray"-by-using-the-divide-and-conquer-approach/20607)
    /// * [最大子序和 c++实现四种解法 暴力法、动态规划、贪心法和分治法 图示讲解](https://leetcode-cn.com/problems/maximum-subarray/solution/zui-da-zi-xu-he-cshi-xian-si-chong-jie-fa-bao-li-f/)
    /// 
    /// ## Submissions
    ///
    /// date=20200620, mem=2.1, mem_beats=92.56, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/355978516/
    /// 
    /// date=20200522, mem=2.1, mem_beats=54, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/179782354/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N log N)
    ///
    /// - 空间：O(log N) 存在log N的stack空间
    pub struct Solution;

    impl Solution {
        pub fn max_sub_array(nums: Vec<i32>) -> i32 {
            fn helper(nums: &[i32], lo: usize, hi: usize) -> i32 {
                if lo >= hi {
                    return nums[lo];
                }
                let mid = (lo + hi) / 2;
                // get max sub sum compare with left sum and right sum
                let sub_max_sum = helper(nums, lo, mid).max(helper(nums, mid + 1, hi));

                // get current max sum with cross left and right
                let (mut sum, mut left_max_sum) = (0, nums[mid]);
                // get left sum from mid to lo
                for i in (lo..=mid).rev() {
                    sum += nums[i];
                    left_max_sum = sum.max(left_max_sum);
                }
                let (mut sum, mut right_max_sum) = (0, nums[mid + 1]);
                // get right sum from mid+1 to hi
                for i in mid + 1..=hi {
                    sum += nums[i];
                    right_max_sum = sum.max(right_max_sum);
                }
                // to compare cross sum and max sub sums
                (left_max_sum + right_max_sum).max(sub_max_sum)
            }

            if nums.is_empty() {
                0
            } else {
                helper(&nums, 0, nums.len() - 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::max_sub_array);
        test(solution_dp_optimized::Solution::max_sub_array);
        test(solution_divide_and_conquer::Solution::max_sub_array);
    }

    fn test<F: Fn(Vec<i32>) -> i32>(func: F) {
        assert_eq!(func(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(func(vec![-2, 1, -3]), 1);
        assert_eq!(func(vec![-2]), -2);
        assert_eq!(func(vec![-2, -1]), -1);
        assert_eq!(func(vec![2, 3, -6, 2, 4]), 6);
    }
}
