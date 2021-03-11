//! 要注意处理下标问题，不需要dp.len = m + 1
pub mod solution_dp {
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
    /// 参考：
    ///
    /// - [C++ DP](https://leetcode.com/problems/unique-paths/discuss/22954/C%2B%2B-DP)
    /// - [告别动态规划，连刷 40 道题，我总结了这些套路，看不懂你打我（万字长文）](https://zhuanlan.zhihu.com/p/91582909)
    ///
    /// ## Submissions
    ///
    /// date=20200530, mem=2.1, mem_beats=100, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/346493605/
    ///
    /// date=20210124, mem=2, mem_beats=59, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140695591/
    ///
    /// date=20210311, mem=2, mem_beats=76, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153810118/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N*M)
    /// - 空间：O(N*M)
    pub struct Solution;

    impl Solution {
        pub fn unique_paths(m: i32, n: i32) -> i32 {
            let (m, n) = (m as usize, n as usize);
            let mut dp = vec![vec![0; n]; m];
            for i in 0..m {
                dp[i][0] = 1;
            }
            for j in 0..n {
                dp[0][j] = 1;
            }
            for i in 1..m {
                for j in 1..n {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
            dp[m - 1][n - 1]
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// dp一维数组表示每个列，每次都先计算了第一行。当二维数组计算`dp[1][1]`时表示使用了`dp[1][0]+dp[0][1]`，可使用`dp[1] = dp[0] + dp[1]`替换
    ///
    /// ### Submissions
    ///
    /// date=20210124, mem=2.1, mem_beats=24, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/140693038/
    ///
    /// date=20210127, mem=2.1, mem_beats=25, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/141524565/
    /// 
    /// date=20210311, mem=2.1, mem_beats=16, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/153816319/
    pub struct Solution;

    impl Solution {
        pub fn unique_paths(m: i32, n: i32) -> i32 {
            let (m, n) = (m as usize, n as usize);
            let mut dp = vec![1; n];
            for _ in 1..m {
                for j in 1..n {
                    dp[j] += dp[j - 1]
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
    fn basic() {
        test(solution_dp::Solution::unique_paths);
        test(solution_dp_optimized::Solution::unique_paths);
    }

    fn test<F: Fn(i32, i32) -> i32>(f: F) {
        assert_eq!(f(3, 2), 3);
        assert_eq!(f(7, 3), 28);
        assert_eq!(f(1, 2), 1);
        assert_eq!(f(1, 1), 1);
    }
}
