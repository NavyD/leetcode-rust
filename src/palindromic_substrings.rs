pub mod solution_dp {
    /// # 思路
    ///
    /// 如果`i..j`是回文子串，那`i == j+1`，且`i+1..j-1`也是回文子串
    ///
    /// dp方程：`dp[i][j] = true if s[i]==s[j] and (j < i or dp[i+1][j-1])`
    ///
    /// 初始化：一个字符必是回文子串`dp[i][i]=true for i in 0..n`
    ///
    /// 注意：for中下标不能相反i<j, j-i<=2,否则j-1负数
    ///
    /// 参考：
    ///
    /// - [两道回文子串的解法（详解中心扩展法）](https://leetcode-cn.com/problems/palindromic-substrings/solution/liang-dao-hui-wen-zi-chuan-de-jie-fa-xiang-jie-zho/)
    ///
    /// ### Submissions
    ///
    /// date=20211014, mem=3.1, mem_beats=5, runtime=4, runtime_beats=23
    ///
    /// date=20211015, mem=3, mem_beats=5, runtime=4, runtime_beats=23
    ///
    /// date=20220320, mem=2.9, mem_beats=12, runtime=4, runtime_beats=25
    ///
    /// date=20220328, mem=3, mem_beats=10, runtime=4, runtime_beats=40
    pub struct Solution;

    impl Solution {
        pub fn count_substrings(s: String) -> i32 {
            let s = s.as_bytes();
            let n = s.len();
            let mut count = 0;

            let mut dp = vec![vec![false; n]; n];
            for j in 0..n {
                for i in 0..=j {
                    if s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                        dp[i][j] = true;
                        count += 1;
                    }
                }
            }
            count
        }
    }
}

pub mod solution_extend {
    /// # 思路
    ///
    /// 选择最中间的 a 作为中心点，往两边扩散
    ///
    /// 参考：
    ///
    /// * [Java solution, 8 lines, extendPalindrome](https://leetcode.com/problems/palindromic-substrings/discuss/105689/Java-solution-8-lines-extendPalindrome)
    /// * [两道回文子串的解法（详解中心扩展法）](https://leetcode-cn.com/problems/palindromic-substrings/solution/liang-dao-hui-wen-zi-chuan-de-jie-fa-xiang-jie-zho/)
    ///
    /// ### Submissions
    ///
    /// date=20211014, mem=2, mem_beats=94, runtime=0, runtime_beats=100
    ///
    /// date=20211015, mem=2, mem_beats=94, runtime=0, runtime_beats=100
    ///
    /// date=20220320, mem=2.1, mem_beats=25, runtime=0, runtime_beats=100
    ///
    /// date=20220328, mem=2.1, mem_beats=40, runtime=0, runtime_beats=100
    pub struct Solution;

    impl Solution {
        pub fn count_substrings(s: String) -> i32 {
            let s = s.as_bytes();
            let n = s.len();

            let extends = |mut left: usize, mut right: usize| {
                let mut count = 0;
                while right < n && s[left] == s[right] {
                    count += 1;
                    if left == 0 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
                count
            };

            (0..n).fold(0, |acc, e| acc + extends(e, e) + extends(e, e + 1))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_dp::Solution::count_substrings);
        test(solution_extend::Solution::count_substrings);
    }

    fn test<F: Fn(String) -> i32>(f: F) {
        assert_eq!(f("abc".to_string()), 3);
        assert_eq!(f("aaa".to_string()), 6);
        assert_eq!(f("aaaaa".to_string()), 15);
    }
}
