pub mod solution_hash {
    /// # 思路
    /// 
    /// 计算两个字符串中每个字母的出现次数并进行比较。
    /// 
    /// 下面是几种细节不同的思路
    ///
    /// 参考：
    ///
    /// ```rust,ignore
    /// pub fn is_anagram(s: String, t: String) -> bool {
    ///     if s.len() != t.len() {
    ///         return false;
    ///     }
    ///     let mut counts = vec![0; 26];
    ///     s.as_bytes()
    ///         .iter()
    ///         .for_each(|&b| counts[(b - b'a') as usize] += 1);
    ///     t.as_bytes()
    ///         .iter()
    ///         .for_each(|&b| counts[(b - b'a') as usize] -= 1);
    ///     counts.iter().all(|&c| c == 0)
    /// }
    /// ```
    /// 
    /// - [画解算法：242. 有效的字母异位词](https://leetcode-cn.com/problems/valid-anagram/solution/hua-jie-suan-fa-242-you-xiao-de-zi-mu-yi-wei-ci-by/)
    /// 
    /// ```java
    /// public boolean isAnagram(String s, String t) {
    ///     if(s.length() != t.length())
    ///         return false;
    ///     int[] alpha = new int[26];
    ///     for(int i = 0; i< s.length(); i++) {
    ///         alpha[s.charAt(i) - 'a'] ++;
    ///         alpha[t.charAt(i) - 'a'] --;
    ///     }
    ///     for(int i=0;i<26;i++)
    ///         if(alpha[i] != 0)
    ///             return false;
    ///     return true;
    /// }
    /// ```
    ///
    /// ### Submissions
    ///
    /// date=20200926, mem=2.2, mem_beats=60.87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/111470082/
    ///
    /// date=20200928, mem=2, mem_beats=95.65, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/112090969/
    /// 
    /// date=20201004, mem=2.2, mem_beats=46.43, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/113152313/
    /// 
    /// ### 复杂度
    ///
    /// - 时间：O(n)
    /// - 空间：O(n)
    pub struct Solution;

    impl Solution {
        pub fn is_anagram(s: String, t: String) -> bool {
            if s.len() != t.len() {
                return false;
            }
            let mut counts = vec![0; 26];
            s.as_bytes()
                .iter()
                .for_each(|b| counts[*b as usize - 'a' as usize] += 1);
            for b in t.as_bytes() {
                let i = *b as usize - 'a' as usize;
                if counts[i] <= 0 {
                    return false;
                }
                counts[i] -= 1;
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        test(solution_hash::Solution::is_anagram);
        test(is_anagram);
    }

    fn test<F: Fn(String, String) -> bool>(func: F) {
        assert!(func("anagram".to_string(), "nagaram".to_string()));
        assert!(!func("rat".to_string(), "car".to_string()));
        assert!(!func("ab".to_string(), "a".to_string()));
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            false
        } else {
            let mut counts = vec![0; 26];
            s.as_bytes().iter().for_each(|b| counts[*b as usize - 'a' as usize] += 1);
            for b in t.as_bytes() {
                let i = *b as usize - 'a' as usize;
                if counts[i] <= 0 {
                    return false;
                }
                counts[i] -= 1;
            }
            true
        }
    }
}
