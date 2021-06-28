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
            let it_s = s.chars();
            let mut it_t = t.chars();
            for ch_s in it_s {
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

pub mod solution_greedy {
    /// # 思路
    ///
    /// 对于follow up，存在大量的s对不变的t。
    ///
    /// 利用贪心算法的思想，只要s中字符在t中下一个出现就可行，
    /// 不需要一个个扫描比较，只需保持s字符快速索引t出next，
    ///
    /// 如何利用s[i]快速找出t, map.get(s[i]) = index，index表示
    /// 在s[i]在t中出现的最近下标，如果index < 0则不s[i]及后面都不
    /// 可能出现了，直接返回。如果index>0则找下一个s[i+1]，要保证
    /// map中s[i],s[i+1]是应该有正确的顺序的
    ///
    /// 如何map index与保持顺序。不能直接用hashmap无序，逆序也不能处
    /// 理重复值
    /// ```ignore
    /// t=abac => 逆序 indexes[a]=0, indexes[b]=1, indexes[c]=3,
    /// s=ba => indexes[b]=1 > indexes[a]=0 // 不成立
    /// ```
    ///
    /// 如何保证顺序，dp[n][26]表示t中字符最近出现的下标index, 如：dp[i][a]=1表示
    /// t[1]=a最近的一个，dp[0..=i][a]=1使0可以索引，当dp[0][s[0]] = i0, dp[i0][s[1]]
    /// dp[i][ai]表示t中0..i的ai字符出现的下标，ai的下标可有`dp[(dp[i-1][a(i-1)])][dp[ai]]`
    /// 由前个dp[i-1][a(i-1)]=index取出ai在a(i-1)后面下标
    ///
    /// 注意对于"abc"在"ahbgdc"上匹配的时候，由于长字符串第一个a的下一个出现a的位置为-1（不出现），
    /// 会导致一个bug。所以在生成数组时在长字符串前插入一个空字符即可。
    ///
    /// ```ignore
    /// s=abc, t=ahbgdc
    ///
    /// c: pos=-1
    /// dp[5][a] = -1, t[5] == a => pos = 5;
    /// dp[4][a] = 5, t[4] != a => pos = 5,
    /// dp[3][a] = 5, t[3] != a => pos = 5,
    /// dp[0][a] = 5, t[0] == a => pos = 0, // t的第1个字符a不能被set
    /// // -----
    /// s=abc, t= ahbgdc
    /// a: pos=-1
    /// dp[6][a] = -1, t[6] != a => pos = -1;
    /// dp[5][a] = 6, t[5] != a => pos = -1,
    /// dp[1][a] = -1, t[1] == a => pos = 1,
    /// dp[0][a] = 1, t[0] != a => pos = 1,
    ///
    /// b: pos=-1
    /// dp[6][b] = -1, t[6] != b => pos = -1;
    /// dp[3][b] = -1, t[3] == b => pos = 3;
    /// dp[2][b] = 3, t[2] != b => pos = 3;
    /// dp[1][b] = 3, t[2] != b => pos = 3;
    ///
    /// c: pos=-1
    /// dp[6][c] = -1, t[6] == c => pos = 6,
    /// dp[5][c] = 6, t[5] != c => pos = 6,
    /// dp[0][c] = 6, t[5] != c => pos = 6,
    ///
    /// =>
    /// s[0]=a
    /// dp[0][a] = 1,
    /// dp[1][b] = 3,
    /// dp[3][c] = 6
    /// ```
    ///
    /// ## Submissions
    ///
    /// date=20200625, mem=4.2, mem_beats=16.67, runtime=4, runtime_beats=7.62, url=https://leetcode.com/submissions/detail/358035266/,
    ///
    /// author=dongzengjie, references=https://leetcode-cn.com/problems/is-subsequence/solution/dui-hou-xu-tiao-zhan-de-yi-xie-si-kao-ru-he-kuai-s/
    ///
    /// ## 复杂度
    ///
    /// 设t为N, s为M
    ///
    /// - 时间：如果将t不作为参数，而是`is_subsequence(s: String)`，初始化t时有O(26*N)，
    /// is_subsequence查询s都存在t中为O(M)，最好时s都不存在t中为O(1)
    /// - 空间：O(26*N)
    pub struct Solution;

    use std::char;

    impl Solution {
        pub fn is_subsequence(s: String, mut t: String) -> bool {
            if s.len() > t.len() {
                return false;
            }
            // init
            t.insert(0, ' ');
            let mut dp = vec![vec![0; 26]; t.len()];
            let index = |i| i as usize - 'a' as usize;
            for c in b'a'..=b'z' {
                let mut pos = -1;
                for (i, ch) in t.char_indices().rev() {
                    dp[i][index(c)] = pos;
                    if ch == char::from_u32(c as u32).unwrap() {
                        pos = i as isize;
                    }
                }
            }
            // index
            let mut i = 0;
            for c in s.chars() {
                i = dp[i as usize][index(c as u8)];
                if i < 0 {
                    return false;
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

            let s = "".to_string();
            let t = "ahbgdc".to_string();
            assert!(Solution::is_subsequence(s, t));
        }
    }
}
