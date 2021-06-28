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
}

pub mod solution_extend {
    /// # 思路
    ///
    /// 从中间向两边扩散，暴力验证回文字符串，
    ///
    /// ![](https://pic.leetcode-cn.com/572db4731d6a0e32ee9c14773ed476068bebb88883335bc7415cb0b43762303a.jpg)
    ///
    /// 注意当中心位置在奇偶长度不一样时，回文形式是不一样的
    ///
    /// ```ignore
    /// s=aba // 中心在b, s[1]
    ///
    /// s=abba // 中心在bb s[1],s[2]
    /// ```
    ///
    /// ## Submissions
    ///
    /// date=20200702, mem=1.9, mem_beats=100, runtime=4, runtime_beats=92.64, url=https://leetcode.com/submissions/detail/360911042/
    ///
    /// rust使用closure时比正常函数更多内存
    ///
    /// author=cdai, references=https://leetcode.com/problems/longest-palindromic-substring/discuss/2928/Very-simple-clean-java-solution/3700
    ///
    /// author=liweiwei1419, references=https://leetcode-cn.com/problems/longest-palindromic-substring/solution/zhong-xin-kuo-san-dong-tai-gui-hua-by-liweiwei1419/
    ///
    /// ## 复杂度
    ///
    /// - 时间：O(N^2)
    /// - 空间：O(1)
    ///
    pub struct Solution;

    impl Solution {
        pub fn longest_palindrome(s: String) -> String {
            let n = s.len();
            let bytes = s.as_bytes();
            let mut max_idx = 0;
            let mut max_len = 0;
            for i in 0..n {
                let odd_s_len = Self::extend(&bytes, i, i);
                let even_s_len = Self::extend(&bytes, i, i + 1);
                if odd_s_len > max_len {
                    max_idx = i - odd_s_len / 2;
                    max_len = odd_s_len;
                }
                if even_s_len > max_len {
                    max_idx = i - even_s_len / 2 + 1;
                    max_len = even_s_len;
                }
            }
            s.get(max_idx..max_idx + max_len).unwrap().to_string()
        }

        fn extend(bytes: &[u8], i: usize, j: usize) -> usize {
            let (mut i, mut j) = (i as isize, j as isize);
            while i >= 0 && j < bytes.len() as isize && bytes[i as usize] == bytes[j as usize] {
                // spread out
                i -= 1;
                j += 1;
            }
            // back to the last valid match
            i += 1;
            j -= 1;
            // usize overflow
            if j < i {
                1
            } else {
                (j - i + 1) as usize
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::longest_palindrome);
        test(solution_extend::Solution::longest_palindrome);
    }

    fn test<F: Fn(String) -> String>(f: F) {
        assert_eq!(f("babad".to_string()), "bab");
        assert_eq!(f("cbbd".to_string()), "bb");
        assert_eq!(f("".to_string()), "");
    }
}
