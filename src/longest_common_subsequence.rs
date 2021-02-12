pub mod solution_dp {
    /// # 思路
    ///
    /// 子问题：当两个字符串最后两个相同时，转化成对前面计算的lcs+1。当不同时，需要比较前个字符串相对另一个的
    /// lcs的最大值。即当前的lcs可通过之前的递推出来
    ///
    /// 设`i,j`表示`s1,s2`的下标，dp[i][j]表示在`s1[i],s2[j]`之前的lcs值。如果`s1[i]==s2[j]`则
    /// `dp[i][j]=dp[i-1][j-1] + 1`，如果`s1[i]!=s2[j]`则`dp[i][j]= max(dp[i-1][j], d[i][j-1]`
    ///
    /// 初始化：
    ///
    /// 当s1,s2中存在空字符串时，返回0
    ///
    /// 当`s1[0]==s2[j]`，j+1..len都将为1，表示当只有一个字符串与s2相比（存在于s2中）时lcs=1。
    /// 当`s2[0]==s1[i]`相似。
    ///
    /// ```ignore
    /// pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    ///     let (m, n) = (text1.len(), text2.len());
    ///     let mut dp = vec![vec![0; n]; m];
    ///     let text1 = text1.as_bytes();
    ///     let text2 = text2.as_bytes();
    ///     for i in 0..m {
    ///         if text1[i] == text2[0] {
    ///             for i in i..m {
    ///                 dp[i][0] = 1;
    ///             }
    ///             break;
    ///         }
    ///     }
    ///     for i in 0..n {
    ///         if text2[i] == text1[0] {
    ///             for i in i..n {
    ///                 dp[0][i] = 1;
    ///             }
    ///             break;
    ///         }
    ///     }
    ///     for i in 1..m {
    ///         for j in 1..n {
    ///             dp[i][j] = if text1[i] == text2[j] {
    ///                 dp[i - 1][j - 1] + 1
    ///             } else {
    ///                 dp[i][j - 1].max(dp[i - 1][j])
    ///             };
    ///         }
    ///     }
    ///     dp[m - 1][n - 1]
    /// }
    ///  ```
    ///
    /// 注意：对于初始化，如果用`dp[i][j]`表示`s1[i-1],s2[j-1]`的lcs值，`dp[0][0]`表示空字符串
    /// 的lcs，就不需要主动初始化了。
    ///
    /// 参考：
    ///
    /// * [动态规划之最长公共子序列（LCS）](https://leetcode-cn.com/problems/longest-common-subsequence/solution/dong-tai-gui-hua-zhi-zui-chang-gong-gong-zi-xu-lie/)
    ///
    /// ### Submissions
    ///
    /// date=20200127, mem=5.8, mem_beats=25, runtime=4, runtime_beats=91, url=https://leetcode-cn.com/submissions/detail/141504192/
    ///
    /// date=20200128, mem=5.8, mem_beats=25, runtime=4, runtime_beats=91, url=https://leetcode-cn.com/submissions/detail/141803274/
    /// 
    /// date=20200212, mem=5.6, mem_beats=84, runtime=4, runtime_beats=91, url=https://leetcode-cn.com/submissions/detail/145366141/
    pub struct Solution;

    impl Solution {
        pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
            let (s1, s2) = (text1.as_bytes(), text2.as_bytes());
            let (m, n) = (s1.len() + 1, s2.len() + 1);
            let mut dp = vec![vec![0; n]; m];
            for i in 1..m {
                for j in 1..n {
                    dp[i][j] = if s1[i - 1] == s2[j - 1] {
                        dp[i - 1][j - 1] + 1
                    } else {
                        dp[i][j - 1].max(dp[i - 1][j])
                    };
                }
            }
            dp[m - 1][n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::longest_common_subsequence);
    }

    fn test<F: Fn(String, String) -> i32>(f: F) {
        assert_eq!(f("abcde".to_string(), "ace".to_string()), 3);
        assert_eq!(f("abc".to_string(), "abc".to_string()), 3);
        assert_eq!(f("abc".to_string(), "def".to_string()), 0);
    }
}
