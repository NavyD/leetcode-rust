pub mod solution_dp {
    /// # 思路
    ///
    /// dp[i][j]表示[i][j]的最小的路径和`dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i - 1][j - 1]`
    ///
    /// 初始值：
    /// `dp[0][0] = dp[1][0] = 0, dp[i][j] = i32.MAX`
    ///
    /// ```ignore
    /// dp[1][1] = min(dp[0][1], dp[1][0]) + grid[0][0] = 1
    /// dp[1][2] = min(dp[0][2], dp[1][1]) + grid[0][1] = 4
    /// dp[1][3] = min(dp[0][3], dp[1][2]) + grid[0][2] = 5
    /// ```
    ///
    /// ## Submissions
    ///
    /// date=20200704, mem=2.2, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361773633/
    ///
    /// date=20210808, mem=2.3, mem_beats=66, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204766255/
    ///
    /// date=20210826, mem=2.4, mem_beats=30, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/211678135/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
            let (m, n) = (grid.len(), grid[0].len());
            let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
            dp[0][1] = 0;
            dp[1][0] = 0;

            for i in 1..=m {
                for j in 1..=n {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i - 1][j - 1];
                }
            }
            dp[m][n]
        }
    }
}

pub mod solution_dp_improved {
    /// # 思路
    ///
    /// dp[j]表示最后第j列的最小路径和，dp[j] = min(dp[j], dp[j - 1]) + grid[i-1][j-1]，
    /// 类似于[Unique Paths II](https://leetcode.com/problems/unique-paths-ii/)
    ///
    /// 初始值 dp[1] = 0
    ///
    /// ## Submissions
    ///
    /// date=20200704, mem=2.4, mem_beats=14.29, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361781144/
    ///
    /// date=20210808, mem=2.4, mem_beats=33, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204770278/
    ///
    /// date=20210826, mem=2.5, mem_beats=6, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/211678964/
    ///
    /// 为何这个mem比起dp[][]还要更多了？
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
            let n = grid[0].len();
            let mut dp = vec![std::i32::MAX; n + 1];
            dp[1] = 0;

            for i in 1..=grid.len() {
                for j in 1..=n {
                    dp[j] = dp[j].min(dp[j - 1]) + grid[i - 1][j - 1];
                }
            }
            dp[n]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::min_path_sum);
        test(solution_dp_improved::Solution::min_path_sum);
    }

    fn test<F: Fn(Vec<Vec<i32>>) -> i32>(f: F) {
        assert_eq!(f(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
        assert_eq!(f(vec![vec![1, 2, 3], vec![4, 5, 6]]), 12);
    }
}
