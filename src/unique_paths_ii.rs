/// 总结
///
/// 1. 在当时没有注意到dp[i][j]与obstacle_grid[i][j]直接相关，而是用
/// `dp[i][j] = obstacle_grid[i-1][j-1] == 0 ? dp[i-1][j] + dp[i][j-1] : 0`，
/// 认为障碍物[i][j]也是可到的，这点是没有想好的
///
/// 2. 考虑初始值不当，使用多个if self判断，没有注意到dp[row+1][col+1]与obstacle_grid
/// 的关系
///
/// 拉跨
///
/// ```ignore
/// pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
///     let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
///     let mut dp = vec![vec![0; n]; m];
///     for i in 1..m {
///         for j in 1..n {
///             if obstacle_grid[i - 1][j] == 0 {
///                 dp[i][j] += dp[i - 1][j];
///             }
///             if obstacle_grid[i][j - 1] == 0 {
///                 dp[i][j] += dp[i][j - 1];
///             }
///         }
///     }
///     todo!()
/// }
/// ```

pub mod solution_dp {
    /// # 思路
    ///
    /// 设dp[i][j]表示能到达[i,j]的路径数，是否能走
    /// `dp[i][j] = obstacle_grid[i][j] == 0 ? dp[i-1][j] + dp[i][j-1] : 0`
    /// 必须先判断 `obstacle_grid[i][j]==1`
    ///
    /// 初始化条件:   `dp[0][0] = obstacle_grid[0][0] == 0 ? 1 : 0`
    ///
    /// 注意`dp[i][j]`在 `obstacle_grid[i][j]`时就不可走了
    ///
    /// 如何解决在i==0||j==0时i-1,j-1的额外判断？
    ///
    /// 由于`dp[i][0] | dp[0][j]`在存在障碍物时后面都是0，需要特判
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
    /// 但如果加大`dp[n+1][m+1]`使`1..=m` `1..=n`时不需要判断obstacle_grid i-1, j-1，
    /// `dp[i][j] = obstacle_grid[i-1][j-1] == 0 ? dp[i-1][j] + dp[i][j-1] : 0`
    /// 如果使得`dp[1][1] = obstacle_grid[0][0] == 0 ? dp[0][1] + dp[1][0] : 0`
    /// 则有`dp[0][1] | dp[0][1] = 1`
    ///
    /// 一个更容易理解的版本
    ///
    /// 参考：
    ///
    /// * [「手画图解」动态规划 思路 63. 不同路径 II](https://leetcode-cn.com/problems/unique-paths-ii/solution/shou-hua-tu-jie-dp-si-lu-by-hyj8/)
    ///
    /// ```ignore
    /// pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    ///     if obstacle_grid[0][0] == 1 {
    ///         return 0;
    ///     }
    ///     let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    ///     let mut dp = vec![vec![0; n]; m];
    ///     // 初始值
    ///     dp[0][0] = 1;
    ///     for i in 1..m {
    ///         // 当前 或 之前没有障碍
    ///         dp[i][0] = if obstacle_grid[i][0] == 0 && dp[i - 1][0] == 1 {
    ///             1
    ///         } else {
    ///             0
    ///         };
    ///     }
    ///     for j in 1..n {
    ///         // 当前 或 之前没有障碍
    ///         dp[0][j] = if obstacle_grid[0][j] == 0 && dp[0][j - 1] == 1 {
    ///             1
    ///         } else{
    ///             0
    ///         };
    ///     }
    ///     for i in 1..m {
    ///         for j in 1..n {
    ///             dp[i][j] = if obstacle_grid[i][j] == 0 {
    ///                 dp[i - 1][j] + dp[i][j - 1]
    ///             } else {
    ///                 0
    ///             };
    ///         }
    ///     }
    ///     dp[m - 1][n - 1]
    /// }
    /// ```
    ///
    /// 参考：
    ///
    /// * [tusizi](https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution)
    /// * [jianchao-li](references=https://leetcode.com/problems/unique-paths-ii/discuss/23252/4ms-O(n)-DP-Solution-in-C%2B%2B-with-Explanations)
    ///
    ///
    /// 子问题：当前位置要左边，上面两个位置推出，如果当前位置是obstacle不可走，则为0
    ///
    /// 设dp[i][j]表示在i,j位置上的路径数，有`dp[i][j] = if grid[i][j]==0 { dp[i-1][j] + dp[i][j-1] } else { 0 }`
    ///
    /// 初始化：当`[0][j]`存在一个障碍时j..len都是0，`[i][0]`同理。如果使用`dp[i][j]`表示i-1,j-1位置，在0,0位置时使用`dp[1][1]`
    /// 如果0,0位置存在障碍，`dp[1][1]=0`，否则`dp[1][1]=1`，要求`dp[0][1]=1 or dp[1][0]=1`。不再需要主动再次初始化
    ///
    /// ## Submissions
    ///
    /// date=20200703, mem=2.1, mem_beats=71.43, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361351029/
    ///
    /// date=20210124, mem=1.9, mem_beats=76, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140720811/
    ///
    /// date=20210127, mem=2, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141515400/
    ///
    /// date=20210525, mem=2, mem_beats=85, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180637540/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
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
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 使用1维数组dp[j]表示在i列时的路径数，由于在行中不断重复列直到最后只要column-1表示
    /// 在最后列和行，
    /// 在第1次访问dp[j]时表示上行obstacle_grid[last_row][j]时的路径数，所以有
    /// `dp[j] = obstacle_grid[row][j] == 0 ? dp[j] + dp[j - 1] : 0`
    ///
    /// 如何处理第0列的问题，当grid[i][0]列中存在障碍=1时，后继的就是dp[i][0]=0，这里
    /// `obstacle_grid[i][j] == 1`与`j > 0`保证了当j==0时如果之前存在障碍时dp[i][0]==0。
    /// 如当只有1列时不存在障碍时dp.length=1只有一个元素dp[n-1]=dp[0]=1。当有障碍时dp[0]被
    /// 后面的` obstacle_grid[i][j] == 1`条件覆盖dp[0]=0
    ///
    /// ```ignore
    /// dp[j] += dp[j - 1];
    /// is
    /// dp[j] = dp[j] + dp[j - 1];
    /// which is new dp[j] = old dp[j] + dp[j-1]
    /// which is current cell = top cell + left cell
    /// ```
    /// 
    /// 下面使用`dp.len=n+1`避免`if j>0`，初始化时不能使用dp[0]=1，dp[1]=1表示
    /// 计算第0列+1，如果使用dp[0]=1，就会在每行重复计算第0列+1，使得结果更大
    /// 
    /// ```
    /// pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    ///     let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    ///     let mut dp = vec![0; n + 1];
    ///     dp[1] = 1;
    ///     for i in 0..m {
    ///         for j in 1..=n {
    ///             if obstacle_grid[i][j - 1] == 0 {
    ///                 dp[j] += dp[j - 1];
    ///             } else {
    ///                 dp[j] = 0;
    ///             }
    ///         }
    ///     }
    ///     dp[n]
    /// }
    /// assert_eq!(
    ///     unique_paths_with_obstacles(vec![
    ///         vec![0, 0, 0],
    ///         vec![0, 1, 0],
    ///         vec![0, 0, 0],
    ///         vec![0, 0, 0],
    ///         vec![0, 0, 0],
    ///     ]),
    ///     7
    /// );
    /// ```
    ///
    /// 参考：
    ///
    /// * [Short JAVA solution](https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution)
    /// * [use n + 1](https://leetcode.com/problems/unique-paths-ii/discuss/23250/Short-JAVA-solution/173411)
    ///
    /// ## Submissions
    ///
    /// date=20200703, mem=2.1, mem_beats=57.14, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/361356420/
    ///
    /// date=20210124, mem=1.9, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140723024/
    ///
    /// date=20210127, mem=2, mem_beats=43, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141516342/
    ///
    /// date=20210525, mem=2, mem_beats=85, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/180638894/
    /// 
    /// date=20210613, mem=2.1, mem_beats=8, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/186290129/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
            let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
            let mut dp = vec![0; n];
            dp[0] = 1;
            for i in 0..m {
                for j in 0..n {
                    if obstacle_grid[i][j] == 1 {
                        // 覆盖 上面来的路径 不存在
                        dp[j] = 0;
                    } else if j > 0 {
                        // 左边来 + 上面的
                        dp[j] += dp[j - 1];
                    }
                }
            }
            dp[n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::unique_paths_with_obstacles);
        test(solution_dp_optimized::Solution::unique_paths_with_obstacles);
    }

    fn test<F: Fn(Vec<Vec<i32>>) -> i32>(f: F) {
        assert_eq!(
            f(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
                vec![0, 0, 0],
            ]),
            7
        );
        assert_eq!(f(vec![vec![0, 1]]), 0);
        assert_eq!(f(vec![vec![1]]), 0);
        assert_eq!(f(vec![vec![0], vec![0], vec![0]]), 1);
    }
}
