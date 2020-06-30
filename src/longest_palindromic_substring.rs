//! Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
//!
//! Example 1:
//!
//! Input: "babad"
//! Output: "bab"
//! Note: "aba" is also a valid answer.
//! Example 2:
//!
//! Input: "cbbd"
//! Output: "bb"

pub mod solution_dp {
    /// # 思路
    ///
    /// 设dp[i][j]表示s[i..=j]是否是回文字符串palindrome，
    /// 将字符串向两边发散，要找更长的substring,必须同时向
    /// 两边+1并比较字符是否相同，所以有
    /// `dp[i][j] = s[i] == s[j] && dp[i - 1][j + 1]`，
    /// 从0..s.len()
    ///
    /// ```ignore
    ///  <---   --->
    /// a b c d c b a
    /// ```
    ///
    /// 考虑到在3个字符内时都有`dp[2][0] = s[2] == s[0] && dp[1][1]`，
    /// `dp[2][1] = s[2] == s[1] && dp[1][2]`，在有2个字符时为dp[1][2]
    /// 是不存在的，是从i-->j向后发散，两个字符只需比较s[i]==s[j]即可，
    /// 即有`dp[i][j] = s[i] == s[j] && (i - j <= 2 || dp[i - 1][j + 1])`
    ///
    /// ## Submissions
    /// 
    /// date=20200630, mem=2.9, mem_beats=9.4, runtime=40, runtime_beats=47.56, url=https://leetcode.com/submissions/detail/360067423/
    /// 
    /// author=jeantimex, references=https://leetcode.com/problems/longest-palindromic-substring/discuss/2921/Share-my-Java-solution-using-dynamic-programming
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(N^2)
    /// - 空间：O(N^2)
    pub struct Solution;

    impl Solution {
        pub fn longest_palindrome(s: String) -> String {
            let n = s.len();
            let mut dp = vec![vec![false; n]; n];
            let bytes = s.as_bytes();
            let mut res = "";
            for i in 0..n {
                for j in (0..=i).rev() {
                    dp[i][j] = bytes[i] == bytes[j] && (i - j <= 2 || dp[i - 1][j + 1]);
                    if dp[i][j] && (i - j + 1) > res.len() {
                        res = s.get(j..=i).unwrap();
                    }
                }
            }
            res.to_string()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
            assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        }
    }
}
