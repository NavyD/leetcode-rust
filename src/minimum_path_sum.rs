//! Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
//!
//! Note: You can only move either down or right at any point in time.
//!
//! Example:
//!
//! Input:
//! [
//!   [1,3,1],
//!   [1,5,1],
//!   [4,2,1]
//! ]
//! Output: 7
//! Explanation: Because the path 1→3→1→1→1 minimizes the sum.

pub mod solution_dp {
    /// # 思路
    ///
    /// dp[i][j]表示[i][j]的最小的路径和dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i - 1][j - 1]
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
    /// author=navyd
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
            let (row_len, col_len) = (grid.len(), grid[0].len());
            let mut dp = vec![vec![std::i32::MAX; col_len + 1]; row_len + 1];
            dp[0][1] = 0;
            for i in 1..=row_len {
                for j in 1..=col_len {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i - 1][j - 1];
                }
            }
            dp[row_len][col_len]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
            assert_eq!(Solution::min_path_sum(grid), 7);
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
    /// author=navyd
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
            let col_len = grid[0].len();
            let mut dp = vec![std::i32::MAX; col_len + 1];
            dp[1] = 0;
            for row in grid {
                for j in 1..=col_len {
                    dp[j] = dp[j].min(dp[j - 1]) + row[j - 1];
                }
            }
            dp[col_len]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn basics() {
            let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
            assert_eq!(Solution::min_path_sum(grid), 7);
        }
    }
}
