//! Given a string s and a string t, check if s is subsequence of t.
//!
//! A subsequence of a string is a new string which is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (ie, "ace" is a subsequence of "abcde" while "aec" is not).
//!
//! Follow up:
//! If there are lots of incoming S, say S1, S2, ... , Sk where k >= 1B, and you want to check one by one to see if T has its subsequence. In this scenario, how would you change your code?
//!
//! Credits:
//! Special thanks to @pbrother for adding this problem and creating all test cases.
//!
//!  
//!
//! Example 1:
//!
//! Input: s = "abc", t = "ahbgdc"
//! Output: true
//! Example 2:
//!
//! Input: s = "axc", t = "ahbgdc"
//! Output: false
//!  
//!
//! Constraints:
//!
//! 0 <= s.length <= 100
//! 0 <= t.length <= 10^4
//! Both strings consists only of lowercase characters.

pub mod solution_two_pointers {
    /// # 思路
    ///
    /// 用two pointers在s中找t中是否存在s的char，s是t的subsequence
    ///
    /// ## Submissions
    ///
    /// date=20200624, mem=2.1, mem_beats=83.33, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/357617777/
    ///
    /// author=navyd
    ///
    /// ## 复杂度
    ///
    /// 设s为N, t为M
    ///
    /// - 时间：O(N+M)
    /// - 空间：O(1)
    pub struct Solution;

    impl Solution {
        pub fn is_subsequence(s: String, t: String) -> bool {
            if s.len() > t.len() {
                return false;
            }
            let mut it_s = s.chars();
            let mut it_t = t.chars();
            while let Some(ch_s) = it_s.next() {
                loop {
                    if let Some(ch_t) = it_t.next() {
                        if ch_t == ch_s {
                            break;
                        }
                    } else {
                        return false;
                    }
                }
            }
            true
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let s = "abc".to_string();
            let t = "ahbgdc".to_string();
            assert!(Solution::is_subsequence(s, t));

            let s = "axc".to_string();
            let t = "ahbgdc".to_string();
            assert!(!Solution::is_subsequence(s, t));
        }
    }
}

pub mod solution_dp {
    /// # 思路
    ///
    /// 设dp[i][j]为在s[0..i]是否是t[0..j]的子字符串，如s=a,t=ab,
    /// dp[0][0]=true表示是s[0]是t[0]的子符串。
    ///
    /// 如果s[i] == t[j]，s[i]是否是t[j]的子字符串应由dp[i-1][j-1]决定，即依赖前面的是否有效
    ///
    /// 如果s[i] != t[j]，s[0..i]与t[0..j-1]比较是否为子字符串
    ///
    /// ```ignore
    /// s=[a,b], t=[c,a,b,e]
    /// s[1] != t[3] // b != e
    /// dp[1][3] = dp[1][2] = true // s[0..1] 是 t[0..2]的子字符串
    /// ```
    ///
    /// 初始化时有dp[0][j]=true，空字符串s是t的子字符串
    ///
    /// ## Submissions
    /// 
    /// date=20200624, mem=2.2, mem_beats=50, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/357640346/
    /// 
    /// author=盛夏与微风, references=https://leetcode-cn.com/problems/is-subsequence/solution/java-dp-by-zxy0917-5/
    /// 
    /// ## 复杂度
    /// 
    /// - 时间：O(NM)
    /// - 空间：O(NM)
    pub struct Solution;

    impl Solution {
        pub fn is_subsequence(s: String, t: String) -> bool {
            if s.is_empty() {
                return true;
            }
            if s.len() > t.len() {
                return false;
            }
            let mut dp = vec![vec![false; t.len() + 1]; s.len() + 1];
            // init
            for i in 0..t.len() {
                dp[0][i] = true;
            }
            for (i, s_ch) in s.char_indices() {
                for (j, t_ch) in t.char_indices() {
                    if s_ch == t_ch {
                        dp[i + 1][j + 1] = dp[i][j];
                    } else {
                        dp[i + 1][j + 1] = dp[i + 1][j]
                    }
                }
            }
            dp[s.len()][t.len()]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            let s = "abc".to_string();
            let t = "ahbgdc".to_string();
            assert!(Solution::is_subsequence(s, t));

            let s = "axc".to_string();
            let t = "ahbgdc".to_string();
            assert!(!Solution::is_subsequence(s, t));

            let s = "".to_string();
            let t = "ahbgdc".to_string();
            assert!(Solution::is_subsequence(s, t));
        }
    }
}
