pub mod solution_dp {
    pub struct Solution;

    impl Solution {
        /// `dp[i][j] = dp[i-1][j-1] if word1[i-1]==word2[j-1]` or
        /// `min(dp[i][j-1]+1, dp[i-1][j]+1, dp[i-1][j-1]+1) if word1[i-1]!=word2[j-1]`
        ///
        /// 参考：
        ///
        /// * [动态规划（Java，最后有同类问题列表）](https://leetcode-cn.com/problems/edit-distance/solution/dong-tai-gui-hua-java-by-liweiwei1419/)
        /// * [C++ O(n)-space DP](https://leetcode.com/problems/edit-distance/discuss/25846/C%2B%2B-O(n)-space-DP)
        /// * [编辑距离](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/)
        /// * [编辑距离 评论解析](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/331399)
        ///
        /// ### Submissions
        ///
        /// date=20210706, mem=3.9, mem_beats=32, runtime=4, runtime_beats=70, url=https://leetcode-cn.com/submissions/detail/192854291/
        ///
        /// date=20210709, mem=4, mem_beats=6, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193860989/
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
            let (len1, len2) = (word1.len(), word2.len());

            let mut dp = vec![vec![0; len2 + 1]; len1 + 1];
            // 初始化：当 word 长度为 0 时非空字符串的长度就是编辑距离
            for i in 1..=len1 {
                dp[i][0] = i;
            }
            for i in 1..=len2 {
                dp[0][i] = i;
            }

            for i in 1..=len1 {
                for j in 1..=len2 {
                    dp[i][j] = if word1[i - 1] == word2[j - 1] {
                        dp[i - 1][j - 1]
                    } else {
                        // insert, delete, replace
                        dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1
                    }
                }
            }
            dp[len1][len2] as i32
        }
    }
}

pub mod solution_dp_optimized {
    /// # 思路
    ///
    /// 参考：
    ///
    /// * [C++ O(n)-space DP](https://leetcode.com/problems/edit-distance/discuss/25846/C%2B%2B-O(n)-space-DP)
    /// * [编辑距离 评论解析](https://leetcode-cn.com/problems/edit-distance/solution/bian-ji-ju-chi-by-leetcode-solution/331399)
    ///
    /// ### Submissions
    ///
    /// date=20210709, mem=2, mem_beats=87, runtime=4, runtime_beats=71, url=https://leetcode-cn.com/submissions/detail/193870878/
    pub struct Solution;

    impl Solution {
        pub fn min_distance(word1: String, word2: String) -> i32 {
            let (word1, word2) = (word1.as_bytes(), word2.as_bytes());
            let (len1, len2) = (word1.len(), word2.len());

            let mut dp = vec![0; len2 + 1];
            for i in 1..=len2 {
                dp[i] = i;
            }

            for i in 1..=len1 {
                let mut pre = dp[0];
                dp[0] = i;
                for j in 1..=len2 {
                    let temp = dp[j];
                    dp[j] = if word1[i - 1] == word2[j - 1] {
                        pre
                    } else {
                        dp[j].min(dp[j - 1]).min(pre) + 1
                    };
                    pre = temp;
                }
            }
            dp[len2] as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        test(solution_dp::Solution::min_distance);
        test(solution_dp_optimized::Solution::min_distance);
    }

    fn test<F: Fn(String, String) -> i32>(f: F) {
        assert_eq!(f("horse".into(), "ros".into()), 3);
        assert_eq!(f("intention".into(), "execution".into()), 5);
        assert_eq!(f("".into(), "a".into()), 1);
    }
}
