/// 第一次写：
///
/// dp[i][j] i表示当前行，j表示当前列，有`dp[i][j] = min(dp[i-1][j], dp[i-1][j-1])`
/// 初始化：dp[0][0]=0, dp[1][0] = triangle[0][0] = 2
///
/// ```ignore
/// pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
///     let (m, n) = (triangle.len() + 1, triangle[triangle.len() - 1].len() + 1);
///     let mut dp = vec![vec![0; n]; m];
///     dp[0][1] = triangle[0][0];
///     for i in 1..m {
///         for j in 1..triangle[i].len() {
///             dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - 1]);
///         }
///     }
///     todo!()
/// }
/// ```
///
/// 在找dp时没有注意关联到`triangle[i][j]`，这是受到了昨天`longest_common_subsequence`的影响。
/// 在初始化的时候写不下去了，自顶向下还有情况没有考虑清楚
pub mod solution_dp {
    /// # 思路
    ///
    /// 子问题：自顶向下`dp[i][j]` i表示当前行，j表示当前列，有`dp[i][j] = min(dp[i-1][j], dp[i-1][j-1]) + triangle[i][j];`
    /// 自底向上：`dp[i][j] = min(dp[i + 1][j], dp[i + 1][j + 1]) + triangle[i][j];`
    ///
    /// 下面是使用额外空间的版本：rust的所有权是决定是否可用原始内存的保证，所以可以复用空间，而不是主动使用额外空间
    ///
    /// m + 1保证最后一行时+dp[m]是+0初始化dp[m], 同理n+1也是在最后一行一列时保证不会出界
    ///
    /// ```
    /// pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    ///     let m = triangle.len();
    ///     let n = triangle[m - 1].len();
    ///     let mut dp = vec![vec![0; n + 1]; m + 1];
    ///     for i in (0..m).rev() {
    ///         // 0..=i表示0..triangle[i].len()-1
    ///         for j in 0..=i {
    ///             dp[i][j] = dp[i + 1][j].min(dp[i + 1][j + 1]) + triangle[i][j];
    ///         }
    ///     }
    ///     dp[0][0]
    /// }
    ///
    /// assert_eq!(minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),11);
    /// ```
    ///
    /// 具体实现使用了自低向上避免初始化问题，由于上一层的`triangle[i].len() = triangle[i+1].len() - 1`，在
    /// 找最小值时不需要判断下标是否出界
    ///
    /// 参考：
    ///
    /// * [Java动态规划两种思路以及代码实现](https://leetcode-cn.com/problems/triangle/solution/javadong-tai-gui-hua-si-lu-yi-ji-dai-ma-shi-xian-b/)
    ///
    /// 这个使用了开始的想法`自顶向下`，实现要考虑边界情况：左端j==0，右端j == i
    ///
    /// * [递归 + 记忆化 + DP，🤷‍♀️ 必须秒懂！](https://leetcode-cn.com/problems/triangle/solution/di-gui-ji-yi-hua-dp-bi-xu-miao-dong-by-sweetiee/)
    ///
    /// ### Submissions
    ///
    /// date=2020128, mem=2.2, mem_beats=57, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141791812/
    ///
    /// date=2020212, mem=2.2, mem_beats=27, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/145363789/
    ///
    /// date=20210222, mem=2.1, mem_beats=59, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/147564298/
    pub struct Solution;

    impl Solution {
        pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
            for i in (0..triangle.len() - 1).rev() {
                for j in 0..=i {
                    triangle[i][j] += triangle[i + 1][j].min(triangle[i + 1][j + 1]);
                }
            }
            triangle[0][0]
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 下一列的值dp[i+1][j]使用dp[j]替换
    ///
    /// 参考：
    ///
    /// * [递归 + 记忆化 + DP，🤷‍♀️ 必须秒懂！](https://leetcode-cn.com/problems/triangle/solution/di-gui-ji-yi-hua-dp-bi-xu-miao-dong-by-sweetiee/)
    ///
    /// ### Submissions
    ///
    /// date=2020128, mem=2.1, mem_beats=64, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141798204/
    ///
    /// date=2020128, mem=2.4, mem_beats=10, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/145364332/
    ///
    /// date=20210222, mem=2.1, mem_beats=59, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/147567568/
    pub struct Solution;

    impl Solution {
        pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
            let n = triangle.last().unwrap().len();
            let mut dp = vec![0; n + 1];
            for i in (0..n).rev() {
                for j in 0..=i {
                    dp[j] = dp[j].min(dp[j + 1]) + triangle[i][j];
                }
            }
            dp[0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::minimum_total);
        test(solution_dp_optimized::Solution::minimum_total);
    }

    fn test<F: Fn(Vec<Vec<i32>>) -> i32>(f: F) {
        assert_eq!(f([[-10]].iter().map(|a| a.to_vec()).collect()), -10);
        assert_eq!(
            f(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
