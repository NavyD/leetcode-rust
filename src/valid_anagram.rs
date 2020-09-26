pub mod solution_hash {
    /// # 思路
    /// 
    /// 参考：
    /// 
    /// ```ignore
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
    /// ### Submissions
    ///
    /// date=20200926, mem=2.2, mem_beats=60.87, runtime=0, runtime_beats=100, url=https://leetcode-cn.com/submissions/detail/111470082/
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

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn basics() {
            assert!(Solution::is_anagram(
                "anagram".to_string(),
                "nagaram".to_string()
            ));
            assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
            assert!(!Solution::is_anagram("ab".to_string(), "a".to_string()));
        }
    }
}
