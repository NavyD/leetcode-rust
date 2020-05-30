//! A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).
//! 
//! The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).
//! 
//! How many possible unique paths are there?
//! 
//! ![test](https://assets.leetcode.com/uploads/2018/10/22/robot_maze.png)
//! 
//! Above is a 7 x 3 grid. How many possible unique paths are there?
//! 
//! Example 1:
//! 
//! Input: m = 3, n = 2
//! Output: 3
//! Explanation:
//! From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
//! 1. Right -> Right -> Down
//! 2. Right -> Down -> Right
//! 3. Down -> Right -> Right
//! Example 2:
//! 
//! Input: m = 7, n = 3
//! Output: 28
//!  
//! 
//! Constraints:
//! 
//! 1 <= m, n <= 100
//! It's guaranteed that the answer will be less than or equal to 2 * 10 ^ 9.

pub struct SolutionByDP;

impl SolutionByDP {
    /// # 思路
    /// 
    /// 由于从某点后到下一点，只能右移或下移到达下一点，假定dp[i][j]是point(i,j)
    /// 的unique paths的数量，则有`dp[i][j]=dp[i-1][j]+dp[i][j-1]`.且有
    /// `dp[0][j]=d[i][0]=1`只能全右或下移
    /// 
    /// ## 问题
    /// 
    /// - `dp[0][j]=d[i][0]=1`如何处理
    /// 
    ///   初始化时将dp[][j]全为1处理，迭代时从(1,1)开始，注意到m=n=1时最小时满足为dp[1][1]=0
    /// 不用特殊处理dp[0][0]=0
    /// 
    /// ## Submissions
    /// 
    /// date=20200530, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/346493605/
    /// 
    /// author=jianchao-li, references=https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP
    /// 
    /// author=帅地, references=https://zhuanlan.zhihu.com/p/91582909
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    /// 
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut paths = vec![vec![1; n]; m];
        for i in 1..m {
            for j in 1..n {
                paths[i][j] = paths[i-1][j] + paths[i][j-1];
            }
        }
        paths[m-1][n-1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn solution_by_dp() {
        assert_eq!(SolutionByDP::unique_paths(3,2), 3);
        assert_eq!(SolutionByDP::unique_paths(7,3), 28);
        assert_eq!(SolutionByDP::unique_paths(1,2), 1);
        assert_eq!(SolutionByDP::unique_paths(1,1), 1);
    }
}