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

/// 错误想法：
///
/// dp[i]表示以i结尾解码的方法数，is_map(s[i]) is_map(s[i-1..=i])
/// 要计算解码方法，如果i是一个映射，则为dp[i-1]+1，如果s[i-1..=i]是
/// 一个映射则dp[i-2]+1，即dp[i]= if is_map(s[i]): dp[i-1]+1 + else if is_map(s[i-1..=i]): dp[i-2]+1
/// dp.len = s.len+1, dp[0] = 0, dp[1] = 1 if is_map(s[0]) else 0
pub mod solution_dp {
    /// # 思路
    ///
    /// 我们只关心位置 i 自己能否形成独立 item 和位置 i 能够与上一位置（i-1）能否形成 item，而不关心 i-1 之前的位置。
    ///
    /// `dp[i]`表示在`s[0..=i]`的解码数：`dp[i] = (is_decoded(s[i-1]) ? dp[i-1] : 0) + is_decoded(s[i-2..i] ? dp[i-2] : 0)`
    /// 可令dp[0] = 1, dp[1] = is_decoded(s[0]) ? 1: 0，用`dp[n+1]`避免`s[i-2]`的判断
    ///
    /// ```ignore
    /// s=226
    ///
    /// dp[0] = 1
    /// dp[1] = 1
    /// dp[2] = (is_decoded(s[1]) ? dp[1] : 0)  + is_decoded(s[0..2]) ? dp[0] : 0 = 2
    /// ```
    ///
    /// 如何判断是否为可解码
    ///
    /// - `s[i]`: `s[i] != '0'`
    /// - `s[i-1..=i]`: `s[i-1] != '0' && s[i] != '0' && ((s[i-1] - '0')*10 + (s[i-2]-'0'))<=26`
    ///
    /// 下面使用字符串解析为u8整数的方式简单处理解码：
    ///
    /// ```
    /// pub fn num_decodings(s: String) -> i32 {
    ///     let n = s.len();
    ///     let mut dp = vec![0; n + 1];
    ///     dp[0] = 1;
    ///     dp[1] = if s[0..1].parse::<u8>().unwrap() != 0 {
    ///         1
    ///     } else {
    ///         0
    ///     };
    ///
    ///     for i in 2..=n {
    ///         let second = s[i - 2..i].parse::<u8>().unwrap();
    ///         if matches!(second, 10..=26) {
    ///             dp[i] += dp[i - 2];
    ///         }
    ///         let first = s[i - 1..i].parse::<u8>().unwrap();
    ///         if matches!(first, 1..=9) {
    ///             dp[i] += dp[i - 1];
    ///         }
    ///     }
    ///     dp[n]
    /// }
    ///
    /// assert_eq!(num_decodings("226".into()), 3);
    /// assert_eq!(num_decodings("340".into()), 0);
    /// ```
    ///
    /// 参考：
    ///
    /// - [使用字符串解析：Java clean DP solution with explanation](https://leetcode.com/problems/decode-ways/discuss/30358/Java-clean-DP-solution-with-explanation)
    /// - [解码方法](https://leetcode-cn.com/problems/decode-ways/solution/jie-ma-fang-fa-by-leetcode-solution-p8np/)
    /// - [C++ 我认为很简单直观的解法](https://leetcode-cn.com/problems/decode-ways/solution/c-wo-ren-wei-hen-jian-dan-zhi-guan-de-jie-fa-by-pr/)
    ///
    /// ## Submissions
    ///
    /// date=20200705, mem=2.1, mem_beats=57.14, runtime=0, runtime_beats=100, url=https://leetcode.com/submissions/detail/362260647/
    ///
    /// date=20210809, mem=2, mem_beats=78, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204884311/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N)
    /// - 空间：O(N)
    pub struct Solution;

    impl Solution {
        pub fn num_decodings(s: String) -> i32 {
            const ZERO: u8 = b'0';
            let s = s.as_bytes();
            let n = s.len();

            let mut dp = vec![0; n + 1];
            dp[0] = 1;
            dp[1] = if s[0] == ZERO { 0 } else { 1 };

            for i in 2..=n {
                if s[i - 1] != ZERO {
                    dp[i] += dp[i - 1];
                }
                if s[i - 2] != ZERO && ((s[i - 2] - ZERO) * 10 + (s[i - 1] - ZERO)) <= 26 {
                    dp[i] += dp[i - 2];
                }
            }
            dp[n]
        }
    }
}

pub mod solution_dp_improved {
    /// # 思路
    ///
    /// 使用两个变量替换`dp[i-1],dp[i-2]`。
    ///
    /// 在解码过程中如果遇到`00, 340`这样无法被解码的，可以提前返回
    ///
    /// ### Submissions
    ///
    /// date=20210809, mem=2, mem_beats=47, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/204893049/
    pub struct Solution;

    impl Solution {
        pub fn num_decodings(s: String) -> i32 {
            const ZERO: u8 = b'0';
            let s = s.as_bytes();

            let (mut cur, mut pre) = (if s[0] == ZERO { 0 } else { 1 }, 1);
            for i in 2..=s.len() {
                let mut next = 0;
                if s[i - 1] != ZERO {
                    next += cur;
                }
                if s[i - 2] != ZERO && ((s[i - 2] - ZERO) * 10 + (s[i - 1] - ZERO)) <= 26 {
                    next += pre;
                }
                pre = cur;
                cur = next;
            }
            cur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::num_decodings);
        test(solution_dp_improved::Solution::num_decodings);
    }

    fn test<F: Fn(String) -> i32>(f: F) {
        assert_eq!(f("12".into()), 2);
        assert_eq!(f("226".into()), 3);
        assert_eq!(f("0".into()), 0);
        assert_eq!(f("340".into()), 0);
        assert_eq!(f("2101".into()), 1);
    }
}
