//! A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
//!
//! The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
//!
//! Now consider if some obstacles are added to the grids. How many unique paths would there be?
//!
//!
//!
//! An obstacle and empty space is marked as 1 and 0 respectively in the grid.
//!
//! Note: m and n will be at most 100.
//!
//! Example 1:
//!
//! Input:
//! [
//!   [0,0,0],
//!   [0,1,0],
//!   [0,0,0]
//! ]
//! Output: 2
//! Explanation:
//! There is one obstacle in the middle of the 3x3 grid above.
//! There are two ways to reach the bottom-right corner:
//! 1. Right -> Right -> Down -> Down
//! 2. Down -> Down -> Right -> Right

pub mod solution_dp {
    /// # 思路
    ///
    /// 设dp[i][j]表示能到达[i,j]的路径数，是否能走
    /// `dp[i][j] = obstacle_grid[i][j] == 0 ? dp[i-1][j] + dp[i][j-1] : 0`
    /// 必须先判断 obstacle_grid[i][j]==1
    ///
    /// 初始化条件:   dp[0][0] = obstacle_grid[0][0] == 0 ? 1 : 0
    ///
    /// 注意dp[i][j]在 obstacle_grid[i][j]时就不可走了
    ///
    /// 如何解决在i==0||j==0时i-1,j-1的额外判断？
    ///
    /// 由于dp[i][0]|dp[0][j]在存在障碍物时后面都是0，需要特判
    ///
    /// ```ignore
    /// let mut dp = vec![vec![0; n]; m];
    /// for i in 0..m {
    ///     for j in 0..n {
    ///         dp[i][j] = if obstacle_grid[i][j] == 1 {
    ///             0
    ///         } else if j > 0 && i > 0 {
    ///             dp[i - 1][j] + dp[i][j - 1]
    ///         } else if i == 0 && j != 0 {
    ///             dp[i][j - 1]
    ///         } else if i != 0 && j == 0 {
    ///             dp[i - 1][j]
    ///         } else {
    ///             1
    ///         }
    ///     }
    /// }
    /// dp[m - 1][n - 1]
    /// ```
    ///
    /// 但如果加大dp[n+1][m+1]使`1..=m` `1..=n`时不需要判断obstacle_grid i-1, j-1，
    /// `dp[i][j] = obstacle_grid[i-1][j-1] == 0 ? dp[i-1][j] + dp[i][j-1] : 0`
    /// 如果使得`dp[1][1] = obstacle_grid[0][0] == 0 ? dp[0][1] + dp[1][0] : 0`
    /// 则有`dp[0][1] | dp[0][1] = 1`
    ///
    /// ## Submissions
    ///
    /// date=20200703, mem=2.1, mem_beats=71.43, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361351029/
    ///
    /// author=navyd
    ///
    /// author=tusizi, references=https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution
    /// 
    /// author=jianchao-li, references=https://leetcode.com/problems/unique-paths-ii/discuss/23252/4ms-O(n)-DP-Solution-in-C%2B%2B-with-Explanations
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let m = obstacle_grid.len();
            let n = obstacle_grid[0].len();
            let mut dp = vec![vec![0; n + 1]; m + 1];
            dp[1][0] = 1;
            for i in 1..=m {
                for j in 1..=n {
                    if obstacle_grid[i - 1][j - 1] == 0 {
                        dp[i][j] = dp[i - 1][j] + dp[i][j - 1]
                    }
                }
            }
            dp[m][n]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let input = vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 7);

            let input = vec![vec![0, 1]];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 0);

            let input = vec![vec![1]];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 0);
        }
    }
}

pub mod solution_dp_improved {
    /// # 思路
    /// 
    /// 使用1维数组dp[j]表示在i列时的路径数，由于在行中不断重复列直到最后只要column-1表示
    /// 在最后列和行，
    /// 在第1次访问dp[j]时表示上行obstacle_grid[last_row][j]时的路径数，所以有
    /// `dp[j] = obstacle_grid[row][j] == 0 ? dp[j] + dp[j - 1] : 0`
    /// 
    /// ```ignore
    /// dp[j] += dp[j - 1];
    /// is
    /// dp[j] = dp[j] + dp[j - 1];
    /// which is new dp[j] = old dp[j] + dp[j-1]
    /// which is current cell = top cell + left cell
    /// ```
    /// 
    /// ## Submissions
    /// 
    /// date=20200703, mem=2.1, mem_beats=57.14, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361356420/
    /// 
    /// author=tusizi, references=https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution
    /// 
    /// author=BirdFrank, references=https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution/22620
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N*M)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let row_len = obstacle_grid.len();
            let col_len = obstacle_grid[0].len();
            let mut dp = vec![0; col_len];
            dp[0] = 1;
            for i in 0..row_len {
                for j in 0..col_len {
                    if obstacle_grid[i][j] == 1 {
                        dp[j] = 0;
                    } else if j > 0 {
                        dp[j] += dp[j - 1];
                    }
                }
            }
            dp[col_len - 1]
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let input = vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 7);

            let input = vec![vec![0, 1]];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 0);

            let input = vec![vec![1]];
            assert_eq!(Solution::unique_paths_with_obstacles(input), 0);
        }
    }
}
