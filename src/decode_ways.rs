//! A message containing letters from A-Z is being encoded to numbers using the following mapping:
//!
//! 'A' -> 1
//! 'B' -> 2
//! ...
//! 'Z' -> 26
//! Given a non-empty string containing only digits, determine the total number of ways to decode it.
//!
//! Example 1:
//!
//! Input: "12"
//! Output: 2
//! Explanation: It could be decoded as "AB" (1 2) or "L" (12).
//! Example 2:
//!
//! Input: "226"
//! Output: 3
//! Explanation: It could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).

/// 总结：
///
/// 设dp[i][j]表示在s[i..=j]中有多少种解码方式，
/// dp[j] = dp[j - 1] + is_decoded(s[j-2..j]) ? 1 : 0
/// dp[0] = 1, dp[1] = dp[0] + is_decoded(s[0..=1]) ? 1 : 0
/// 122223
/// dp[0] = 1
/// dp[1] = dp[0] + is_decoded(12) ? 1 : 0 = 2
/// dp[2] = dp[1] + is_decoded(22) ? 1 : 0 = 3  // 1,2,2        12,2        1,22
/// dp[3] = dp[2] + is_decoded(26) ? 1 : 0 = 4  // 1,2,2,2      12,2,2      1,22,2      12,22       1,2,22
/// dp[4] =                                     // 1,2,2,2,2    12,2,2,2    1,22,2,2    12,22,2     1,2,22,2    1,2,2,22    12,2,22    1,22,22
/// dp[4] =                                     // 1,2,2,6,2,3  12,2,6,2,3  1,22,6,2,3  12,26,2,3   1,2,26,2,3  
///
/// 一开始是有注意到dp[j]与j-1, j-2的关系的，在举例发现不对时，被举例规则误导，开始找结果的规
/// 则，导致过于复杂放弃，只要稍微想下就可以得出正确结果的

pub mod solution_dp {
    /// # 思路
    /// dp[i]表示在s[0..=i]的解码方式：dp[i] = (is_decoded(s[i-1]) ? dp[i-1] : 0) + is_decoded(s[i-2..i] ? dp[i-2] : 0)
    /// 可令dp[0] = 1, dp[1] = is_decoded(s[0]) ? 1: 0，用dp[n+1]避免s[i-2]的判断
    ///
    /// ```ignore
    /// s=226
    ///
    /// dp[0] = 1
    /// dp[1] = 1
    /// dp[2] = (is_decoded(s[1]) ? dp[1] : 0)  + is_decoded(s[0..2]) ? dp[0] : 0 = 2
    /// ```
    ///
    /// ## Submissions
    ///
    /// date=20200705, mem=2.1, mem_beats=57.14, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/362260647/
    ///
    /// author=yfcheng, references=https://leetcode.com/problems/decode-ways/discuss/30358/Java-clean-DP-solution-with-explanation
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn num_decodings(s: String) -> i32 {
            let mut dp = vec![0; s.len() + 1];
            dp[0] = 1;
            dp[1] = if s.get(0..1).unwrap().parse::<u8>().unwrap() != 0 {
                1
            } else {
                0
            };
            for i in 2..s.len() + 1 {
                let second = s.get(i - 2..i).unwrap().parse::<u8>().unwrap();
                if second >= 10 && second <= 26 {
                    dp[i] += dp[i - 2];
                }
                let first = s.get(i - 1..i).unwrap().parse::<u8>().unwrap();
                if first >= 1 && first <= 9 {
                    dp[i] += dp[i - 1];
                }
            }
            dp[s.len()]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn basics() {
            assert_eq!(Solution::num_decodings("226".to_string()), 3);
            assert_eq!(Solution::num_decodings("12".to_string()), 2);
        }
    }
}
